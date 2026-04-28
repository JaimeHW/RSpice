//! Digital Waveform State and Rendering
//!
//! Provides digital signal display for mixed-signal simulation.
//! Digital waveforms are rendered as step transitions (HIGH/LOW levels) rather
//! than analog traces, with proper logic-level thresholds and bus grouping.
//!
//! This follows standard industry patterns for
//! displaying digital signals in transient simulation results.
//!
//! # Usage
//!
//! ```ignore
//! let config = DigitalWaveformConfig::cmos_3v3();
//! let signal = DigitalSignal::from_analog(&time, &voltage, &config);
//!
//! // Render in waveform viewer
//! render_digital_waveform(&signal, &render_context);
//! ```

use serde::{Deserialize, Serialize};

// =============================================================================
// Digital Waveform Configuration
// =============================================================================

/// Logic family presets for threshold configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum LogicFamily {
    /// CMOS 3.3V (VIH=2.0V, VIL=0.8V)
    #[default]
    Cmos3v3,
    /// CMOS 5V (VIH=3.5V, VIL=1.5V)
    Cmos5v,
    /// CMOS 1.8V (VIH=1.17V, VIL=0.63V)
    Cmos1v8,
    /// CMOS 1.2V (VIH=0.78V, VIL=0.42V)
    Cmos1v2,
    /// LVCMOS 3.3V
    Lvcmos3v3,
    /// TTL (VIH=2.0V, VIL=0.8V)
    Ttl,
    /// LVDS (differential, VIH=+100mV, VIL=-100mV relative to common-mode)
    Lvds,
    /// Custom thresholds
    Custom,
}

impl LogicFamily {
    /// Get threshold voltages (VIL, VIH) for this logic family
    pub fn thresholds(&self) -> (f64, f64) {
        match self {
            LogicFamily::Cmos3v3 => (0.8, 2.0),
            LogicFamily::Cmos5v => (1.5, 3.5),
            LogicFamily::Cmos1v8 => (0.63, 1.17),
            LogicFamily::Cmos1v2 => (0.42, 0.78),
            LogicFamily::Lvcmos3v3 => (0.8, 2.0),
            LogicFamily::Ttl => (0.8, 2.0),
            LogicFamily::Lvds => (-0.1, 0.1), // Relative to common-mode
            LogicFamily::Custom => (0.8, 2.0),
        }
    }

    /// Get supply voltage for this logic family
    pub fn vdd(&self) -> f64 {
        match self {
            LogicFamily::Cmos3v3 => 3.3,
            LogicFamily::Cmos5v => 5.0,
            LogicFamily::Cmos1v8 => 1.8,
            LogicFamily::Cmos1v2 => 1.2,
            LogicFamily::Lvcmos3v3 => 3.3,
            LogicFamily::Ttl => 5.0,
            LogicFamily::Lvds => 1.2,
            LogicFamily::Custom => 3.3,
        }
    }
}

/// Configuration for digital signal interpretation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalWaveformConfig {
    /// Logic family preset
    pub logic_family: LogicFamily,
    /// Low threshold voltage (VIL) - below this = logic 0
    pub v_low: f64,
    /// High threshold voltage (VIH) - above this = logic 1
    pub v_high: f64,
    /// Hysteresis for state transitions (prevents glitches)
    pub hysteresis: f64,
    /// Minimum pulse width to register (seconds) - filters glitches
    pub min_pulse_width: f64,
    /// Whether signal is inverted
    pub inverted: bool,
}

impl Default for DigitalWaveformConfig {
    fn default() -> Self {
        Self::cmos_3v3()
    }
}

impl DigitalWaveformConfig {
    /// Create config for CMOS 3.3V logic
    pub fn cmos_3v3() -> Self {
        let (vil, vih) = LogicFamily::Cmos3v3.thresholds();
        Self {
            logic_family: LogicFamily::Cmos3v3,
            v_low: vil,
            v_high: vih,
            hysteresis: 0.1,
            min_pulse_width: 0.0,
            inverted: false,
        }
    }

    /// Create config for CMOS 1.8V logic
    pub fn cmos_1v8() -> Self {
        let (vil, vih) = LogicFamily::Cmos1v8.thresholds();
        Self {
            logic_family: LogicFamily::Cmos1v8,
            v_low: vil,
            v_high: vih,
            hysteresis: 0.05,
            min_pulse_width: 0.0,
            inverted: false,
        }
    }

    /// Create config for TTL logic
    pub fn ttl() -> Self {
        let (vil, vih) = LogicFamily::Ttl.thresholds();
        Self {
            logic_family: LogicFamily::Ttl,
            v_low: vil,
            v_high: vih,
            hysteresis: 0.1,
            min_pulse_width: 0.0,
            inverted: false,
        }
    }

    /// Create custom config with specified thresholds
    pub fn custom(v_low: f64, v_high: f64) -> Self {
        Self {
            logic_family: LogicFamily::Custom,
            v_low,
            v_high,
            hysteresis: (v_high - v_low) * 0.1,
            min_pulse_width: 0.0,
            inverted: false,
        }
    }

    /// Create from logic family preset
    pub fn from_family(family: LogicFamily) -> Self {
        let (vil, vih) = family.thresholds();
        Self {
            logic_family: family,
            v_low: vil,
            v_high: vih,
            hysteresis: (vih - vil) * 0.1,
            min_pulse_width: 0.0,
            inverted: false,
        }
    }

    /// Set hysteresis for noise immunity
    pub fn with_hysteresis(mut self, h: f64) -> Self {
        self.hysteresis = h;
        self
    }

    /// Set minimum pulse width filter
    pub fn with_glitch_filter(mut self, min_width: f64) -> Self {
        self.min_pulse_width = min_width;
        self
    }

    /// Set inversion
    pub fn inverted(mut self, inv: bool) -> Self {
        self.inverted = inv;
        self
    }
}

// =============================================================================
// Digital Logic State
// =============================================================================

/// Digital logic state value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LogicState {
    /// Logic low (0)
    Low,
    /// Logic high (1)
    High,
    /// Unknown/undefined (X) - between thresholds
    Unknown,
    /// High impedance (Z)
    HighZ,
}

impl LogicState {
    /// Get display character
    pub fn char(&self) -> char {
        match self {
            LogicState::Low => '0',
            LogicState::High => '1',
            LogicState::Unknown => 'X',
            LogicState::HighZ => 'Z',
        }
    }

    /// Get color for rendering
    pub fn color(&self) -> &'static str {
        match self {
            LogicState::Low => "#22c55e",     // Green for low
            LogicState::High => "#ef4444",    // Red for high
            LogicState::Unknown => "#f59e0b", // Amber for unknown
            LogicState::HighZ => "#6b7280",   // Gray for hi-z
        }
    }

    /// Get vertical position (0.0 = bottom, 1.0 = top)
    pub fn y_position(&self) -> f64 {
        match self {
            LogicState::Low => 0.1,
            LogicState::High => 0.9,
            LogicState::Unknown => 0.5,
            LogicState::HighZ => 0.5,
        }
    }

    /// Determine state from voltage
    pub fn from_voltage(v: f64, config: &DigitalWaveformConfig) -> Self {
        let state = if v <= config.v_low {
            LogicState::Low
        } else if v >= config.v_high {
            LogicState::High
        } else {
            LogicState::Unknown
        };

        if config.inverted {
            state.invert()
        } else {
            state
        }
    }

    /// Invert the logic state
    pub fn invert(&self) -> Self {
        match self {
            LogicState::Low => LogicState::High,
            LogicState::High => LogicState::Low,
            LogicState::Unknown => LogicState::Unknown,
            LogicState::HighZ => LogicState::HighZ,
        }
    }
}

// =============================================================================
// Digital Signal Edge
// =============================================================================

/// Digital signal edge/transition
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DigitalEdge {
    /// Time of edge
    pub time: f64,
    /// State before edge
    pub from_state: LogicState,
    /// State after edge
    pub to_state: LogicState,
}

impl DigitalEdge {
    /// Create a new edge
    pub fn new(time: f64, from: LogicState, to: LogicState) -> Self {
        Self {
            time,
            from_state: from,
            to_state: to,
        }
    }

    /// Is this a rising edge (0 -> 1)?
    pub fn is_rising(&self) -> bool {
        self.from_state == LogicState::Low && self.to_state == LogicState::High
    }

    /// Is this a falling edge (1 -> 0)?
    pub fn is_falling(&self) -> bool {
        self.from_state == LogicState::High && self.to_state == LogicState::Low
    }
}

// =============================================================================
// Digital Signal
// =============================================================================

/// Represents a digital signal derived from analog simulation data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignal {
    /// Signal name
    pub name: String,
    /// Configuration used for digitization
    pub config: DigitalWaveformConfig,
    /// List of edges (state transitions)
    pub edges: Vec<DigitalEdge>,
    /// Initial state at t=0
    pub initial_state: LogicState,
    /// Time range
    pub time_start: f64,
    pub time_end: f64,
}

impl DigitalSignal {
    /// Create a digital signal from analog waveform data
    ///
    /// # Arguments
    /// * `name` - Signal name
    /// * `times` - Time values
    /// * `voltages` - Voltage values
    /// * `config` - Digitization configuration
    pub fn from_analog(
        name: &str,
        times: &[f64],
        voltages: &[f64],
        config: &DigitalWaveformConfig,
    ) -> Self {
        if times.is_empty() || times.len() != voltages.len() {
            return Self {
                name: name.to_string(),
                config: config.clone(),
                edges: vec![],
                initial_state: LogicState::Unknown,
                time_start: 0.0,
                time_end: 0.0,
            };
        }

        let samples: Vec<(f64, f64)> = times
            .iter()
            .copied()
            .zip(voltages.iter().copied())
            .filter(|(time, voltage)| time.is_finite() && voltage.is_finite())
            .collect();

        if samples.is_empty() {
            return Self {
                name: name.to_string(),
                config: config.clone(),
                edges: vec![],
                initial_state: LogicState::Unknown,
                time_start: 0.0,
                time_end: 0.0,
            };
        }

        let time_start = samples[0].0;
        let time_end = samples.last().map(|(time, _)| *time).unwrap_or(time_start);

        // Determine initial state
        let initial_state = LogicState::from_voltage(samples[0].1, config);

        // Find edges using hysteresis
        let mut edges = Vec::new();
        let mut current_state = initial_state;
        let mut last_transition_time = time_start;
        let mut prev_time = time_start;

        for &(edge_time, voltage) in samples.iter().skip(1) {
            if edge_time <= prev_time {
                continue;
            }
            prev_time = edge_time;

            let new_state = Self::evaluate_with_hysteresis(voltage, current_state, config);

            if new_state != current_state {
                // Apply glitch filter
                if config.min_pulse_width > 0.0 {
                    let pulse_duration = edge_time - last_transition_time;
                    if pulse_duration < config.min_pulse_width {
                        continue; // Filter out short pulse
                    }
                }

                edges.push(DigitalEdge::new(edge_time, current_state, new_state));
                current_state = new_state;
                last_transition_time = edge_time;
            }
        }

        Self {
            name: name.to_string(),
            config: config.clone(),
            edges,
            initial_state,
            time_start,
            time_end,
        }
    }

    /// Evaluate state with hysteresis to prevent oscillation at thresholds
    fn evaluate_with_hysteresis(
        voltage: f64,
        current_state: LogicState,
        config: &DigitalWaveformConfig,
    ) -> LogicState {
        let h = config.hysteresis;

        match current_state {
            LogicState::Low => {
                // Must exceed high threshold + hysteresis to switch to high
                if voltage >= config.v_high + h {
                    LogicState::High
                } else if voltage >= config.v_low + h && voltage < config.v_high {
                    LogicState::Unknown
                } else {
                    LogicState::Low
                }
            }
            LogicState::High => {
                // Must go below low threshold - hysteresis to switch to low
                if voltage <= config.v_low - h {
                    LogicState::Low
                } else if voltage > config.v_low && voltage <= config.v_high - h {
                    LogicState::Unknown
                } else {
                    LogicState::High
                }
            }
            LogicState::Unknown | LogicState::HighZ => {
                // Standard threshold check from undefined state
                LogicState::from_voltage(voltage, config)
            }
        }
    }

    /// Get state at a specific time
    pub fn state_at(&self, time: f64) -> LogicState {
        if time < self.time_start {
            return LogicState::Unknown;
        }

        let mut state = self.initial_state;
        for edge in &self.edges {
            if edge.time > time {
                break;
            }
            state = edge.to_state;
        }
        state
    }

    /// Get all edge times for rendering
    pub fn edge_times(&self) -> Vec<f64> {
        self.edges.iter().map(|e| e.time).collect()
    }

    /// Count rising edges
    pub fn rising_edge_count(&self) -> usize {
        self.edges.iter().filter(|e| e.is_rising()).count()
    }

    /// Count falling edges
    pub fn falling_edge_count(&self) -> usize {
        self.edges.iter().filter(|e| e.is_falling()).count()
    }

    /// Calculate frequency from edge count (approximate)
    pub fn approx_frequency(&self) -> Option<f64> {
        let duration = self.time_end - self.time_start;
        if duration <= 0.0 {
            return None;
        }

        let cycles = self.rising_edge_count();
        if cycles < 2 {
            return None;
        }

        Some(cycles as f64 / duration)
    }

    /// Generate render segments for waveform display
    ///
    /// Returns a list of (start_time, end_time, state) tuples
    pub fn render_segments(&self) -> Vec<(f64, f64, LogicState)> {
        let mut segments = Vec::new();

        if self.edges.is_empty() {
            segments.push((self.time_start, self.time_end, self.initial_state));
            return segments;
        }

        // Initial segment
        let first_edge = &self.edges[0];
        if first_edge.time > self.time_start {
            segments.push((self.time_start, first_edge.time, self.initial_state));
        }

        // Segments between edges
        for i in 0..self.edges.len() {
            let start_time = self.edges[i].time;
            let end_time = if i + 1 < self.edges.len() {
                self.edges[i + 1].time
            } else {
                self.time_end
            };
            let state = self.edges[i].to_state;

            segments.push((start_time, end_time, state));
        }

        segments
    }
}

// =============================================================================
// Digital Bus
// =============================================================================

/// Represents a group of digital signals as a bus (e.g., DATA[7:0])
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalBus {
    /// Bus name
    pub name: String,
    /// Individual bit signals (index 0 = LSB)
    pub signals: Vec<DigitalSignal>,
    /// Display format for bus values
    pub display_radix: BusRadix,
}

/// Display radix for bus values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum BusRadix {
    /// Binary (e.g., "10101011")
    Binary,
    /// Hexadecimal (e.g., "AB")
    #[default]
    Hex,
    /// Decimal unsigned (e.g., "171")
    Decimal,
    /// Decimal signed (e.g., "-85")
    SignedDecimal,
    /// ASCII character
    Ascii,
}

impl DigitalBus {
    /// Create a new digital bus
    pub fn new(name: &str, signals: Vec<DigitalSignal>) -> Self {
        Self {
            name: name.to_string(),
            signals,
            display_radix: BusRadix::Hex,
        }
    }

    /// Get bus width in bits
    pub fn width(&self) -> usize {
        self.signals.len()
    }

    /// Get bus value at a specific time
    pub fn value_at(&self, time: f64) -> Option<u64> {
        let mut value: u64 = 0;

        for (i, signal) in self.signals.iter().enumerate() {
            match signal.state_at(time) {
                LogicState::High => value |= 1 << i,
                LogicState::Low => {} // Bit already 0
                LogicState::Unknown | LogicState::HighZ => return None, // Unknown value
            }
        }

        Some(value)
    }

    /// Format bus value for display
    pub fn format_value(&self, value: u64) -> String {
        match self.display_radix {
            BusRadix::Binary => format!("{:0width$b}", value, width = self.width()),
            BusRadix::Hex => format!("{:0width$X}", value, width = self.width().div_ceil(4)),
            BusRadix::Decimal => format!("{}", value),
            BusRadix::SignedDecimal => {
                let width = self.width();
                if width > 0 && value & (1 << (width - 1)) != 0 {
                    // Negative number - sign extend
                    let mask = (1u64 << width) - 1;
                    let signed = value as i64 | !mask as i64;
                    format!("{}", signed)
                } else {
                    format!("{}", value as i64)
                }
            }
            BusRadix::Ascii => {
                if value < 128 {
                    let c = value as u8 as char;
                    if c.is_ascii_graphic() || c == ' ' {
                        format!("'{}'", c)
                    } else {
                        format!("0x{:02X}", value)
                    }
                } else {
                    format!("0x{:02X}", value)
                }
            }
        }
    }

    /// Get all value change times
    pub fn change_times(&self) -> Vec<f64> {
        let mut times: Vec<f64> = self
            .signals
            .iter()
            .flat_map(|s| s.edge_times())
            .filter(|time| time.is_finite())
            .collect();
        times.sort_by(f64::total_cmp);
        times.dedup();
        times
    }
}

// =============================================================================
// Tests
// =============================================================================
