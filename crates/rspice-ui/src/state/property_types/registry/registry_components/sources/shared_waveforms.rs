//! Waveform families whose voltage and current sheets differ only in unit and
//! default instance name.
//!
//! SFFM, AM, PAT, and TRNOISE all take the same arguments whichever quantity
//! they drive — ngspice even keeps the `V`-prefixed spellings (VO, VA, VHI) on
//! current sources — so each family is built once here and stamped twice. That
//! is the only way the two sheets cannot drift apart.

use super::*;

/// The quantity a source drives. Only the unit, the default instance name, and
/// the current-source amplitude defaults depend on it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Driven {
    Voltage,
    Current,
}

impl Driven {
    fn unit(self) -> &'static str {
        match self {
            Driven::Voltage => "V",
            Driven::Current => "A",
        }
    }

    fn default_name(self) -> &'static str {
        match self {
            Driven::Voltage => "V1",
            Driven::Current => "I1",
        }
    }

    /// A 1 V swing is a sensible placeholder; 1 A is not.
    fn full_scale(self) -> f64 {
        match self {
            Driven::Voltage => 1.0,
            Driven::Current => 1e-3,
        }
    }

    fn has_series_resistance(self) -> bool {
        self == Driven::Voltage
    }
}

fn instance_name(driven: Driven) -> PropertyDefinition {
    PropertyDefinition::new("name")
        .with_display_name("Instance Name")
        .with_type(PropertyType::String)
        .with_default(PropertyValue::string(driven.default_name()))
        .with_order(0)
        .with_category("Instance")
        .required()
}

impl PropertyRegistry {
    /// SFFM(VO VA FC MDI FM TD PHASEM PHASEC).
    ///
    /// The fifth field is `fm`, which is what `SourceSpec::Sffm` and ngspice
    /// both call the modulating frequency. This sheet called it `FS` until the
    /// app was reconciled with the engine; a card authored under the old
    /// spelling is renamed as its component loads, so nothing downstream has
    /// two names to accept.
    ///
    /// FC and FM carry explicit defaults rather than ngspice's tstop-derived
    /// ones: a source placed on a schematic has to mean the same thing under
    /// every `.tran` stop time it is ever swept with.
    pub(super) fn create_sffm_sheet(driven: Driven) -> PropertySheet {
        let unit = driven.unit();
        let mut sheet = PropertySheet::new();
        sheet.add(instance_name(driven));

        sheet.add(
            PropertyDefinition::new("vo")
                .with_display_name("Offset (VO)")
                .with_description("Output offset")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(10)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("va")
                .with_display_name("Amplitude (VA)")
                .with_description("Carrier amplitude")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(driven.full_scale()))
                .with_unit(unit)
                .with_order(11)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("fc")
                .with_display_name("Carrier Freq (FC)")
                .with_description("Carrier frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(12)
                .with_category("SFFM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("mdi")
                .with_display_name("Mod Index (MDI)")
                .with_description("Modulation index; limited to FC/FM during simulation")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(13)
                .with_category("SFFM"),
        );
        sheet.add(
            PropertyDefinition::new("fm")
                .with_display_name("Modulating Freq (FM)")
                .with_description("Signal (modulating) frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e3))
                .with_unit("Hz")
                .with_order(14)
                .with_category("SFFM"),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Output is exactly zero before this time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(15)
                .with_category("SFFM"),
        );
        sheet.add(
            PropertyDefinition::new("phasem")
                .with_display_name("Mod Phase (PHASEM)")
                .with_description("Modulation phase in degrees")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(16)
                .with_category("SFFM"),
        );
        sheet.add(
            PropertyDefinition::new("phasec")
                .with_display_name("Carrier Phase (PHASEC)")
                .with_description("Carrier phase in degrees")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(17)
                .with_category("SFFM"),
        );

        Self::finish_source_sheet(&mut sheet, driven);
        sheet
    }

    /// AM(VO VMO VMA FM FC TD PHASEM PHASEC).
    ///
    /// The envelope is `VMO + VMA*sin(2*pi*FM*t + PHASEM)`, so leaving VMO at 0
    /// gives full (100 %) modulation with carrier suppression at the envelope
    /// zero crossings; raise it above VMA for an ordinary broadcast-style AM.
    pub(super) fn create_am_sheet(driven: Driven) -> PropertySheet {
        let unit = driven.unit();
        let mut sheet = PropertySheet::new();
        sheet.add(instance_name(driven));

        sheet.add(
            PropertyDefinition::new("vo")
                .with_display_name("Offset (VO)")
                .with_description("Overall output offset")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(10)
                .with_category("AM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("vmo")
                .with_display_name("Mod Offset (VMO)")
                .with_description("Envelope offset; 0 gives full-depth modulation")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(11)
                .with_category("AM"),
        );
        sheet.add(
            PropertyDefinition::new("vma")
                .with_display_name("Mod Amplitude (VMA)")
                .with_description("Envelope swing about the modulation offset")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(driven.full_scale()))
                .with_unit(unit)
                .with_order(12)
                .with_category("AM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("fm")
                .with_display_name("Mod Freq (FM)")
                .with_description("Modulating frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e3))
                .with_unit("Hz")
                .with_order(13)
                .with_category("AM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("fc")
                .with_display_name("Carrier Freq (FC)")
                .with_description("Carrier frequency")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e6))
                .with_unit("Hz")
                .with_order(14)
                .with_category("AM")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Output is exactly zero before this time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(15)
                .with_category("AM"),
        );
        sheet.add(
            PropertyDefinition::new("phasem")
                .with_display_name("Mod Phase (PHASEM)")
                .with_description("Modulation phase in degrees")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(16)
                .with_category("AM"),
        );
        sheet.add(
            PropertyDefinition::new("phasec")
                .with_display_name("Carrier Phase (PHASEC)")
                .with_description("Carrier phase in degrees")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_unit("°")
                .with_range(-360.0, 360.0)
                .with_order(17)
                .with_category("AM"),
        );

        Self::finish_source_sheet(&mut sheet, driven);
        sheet
    }

    /// PAT(VHI VLO TD TR TF TSAMPLE DATA [R=n]).
    ///
    /// TR, TF, and TSAMPLE must all be strictly positive and DATA must be a
    /// B-prefixed bit string; the engine rejects the deck otherwise, so the
    /// defaults here are already valid.
    pub(super) fn create_pat_sheet(driven: Driven) -> PropertySheet {
        let unit = driven.unit();
        let mut sheet = PropertySheet::new();
        sheet.add(instance_name(driven));

        sheet.add(
            PropertyDefinition::new("vhi")
                .with_display_name("High Level (VHI)")
                .with_description("Output for a 1 bit")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(driven.full_scale()))
                .with_unit(unit)
                .with_order(10)
                .with_category("Pattern")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("vlo")
                .with_display_name("Low Level (VLO)")
                .with_description("Output for a 0 bit")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(11)
                .with_category("Pattern")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("data")
                .with_display_name("Bit Pattern (DATA)")
                .with_description("B-prefixed bit string, for example b0101")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string("b0101"))
                .with_order(12)
                .with_category("Pattern")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("tsample")
                .with_display_name("Bit Interval (TSAMPLE)")
                .with_description("Time held per bit; must be greater than zero")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-6))
                .with_unit("s")
                .with_order(13)
                .with_category("Pattern")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time (TD)")
                .with_description("Delay before the first bit; may be negative")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(14)
                .with_category("Pattern"),
        );
        sheet.add(
            PropertyDefinition::new("tr")
                .with_display_name("Rise Time (TR)")
                .with_description("Low-to-high edge duration; must be greater than zero")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(15)
                .with_category("Pattern"),
        );
        sheet.add(
            PropertyDefinition::new("tf")
                .with_display_name("Fall Time (TF)")
                .with_description("High-to-low edge duration; must be greater than zero")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1e-9))
                .with_unit("s")
                .with_order(16)
                .with_category("Pattern"),
        );
        sheet.add(
            PropertyDefinition::new("repeat_count")
                .with_display_name("Repeat Count (R)")
                .with_description("Extra pattern repetitions; -1 repeats forever")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_range(-1.0, 1e9)
                .with_order(17)
                .with_category("Pattern"),
        );

        Self::finish_source_sheet(&mut sheet, driven);
        sheet
    }

    /// TRNOISE(NA NT NALPHA NAMP RTSAM RTSCAPT RTSEMT).
    ///
    /// The parser reads all seven positionally and defaults every omitted one
    /// to zero (`netlist/parser/source_specs.rs` `parse_trnoise_spec`, lines
    /// 562-626). NT must be positive whenever NA or NAMP is nonzero, 1/f
    /// injection needs 0 <= NALPHA < 2, and the RTS triple is all-or-nothing.
    pub(super) fn create_trnoise_sheet(driven: Driven) -> PropertySheet {
        let unit = driven.unit();
        let mut sheet = PropertySheet::new();
        sheet.add(instance_name(driven));

        sheet.add(
            PropertyDefinition::new("dc")
                .with_display_name(match driven {
                    Driven::Voltage => "DC Voltage",
                    Driven::Current => "DC Current",
                })
                .with_description("Operating-point value; the noise itself is zero-mean")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(1)
                .with_category("DC"),
        );
        sheet.add(
            PropertyDefinition::new("na")
                .with_display_name("White RMS Amplitude")
                .with_description("Gaussian white-noise RMS amplitude (0 disables)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1n"))
                .with_unit(unit)
                .with_order(10)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("nt")
                .with_display_name("Sample Interval")
                .with_description("Noise sample timestep (must be > 0 when noise is enabled)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::expression("1u"))
                .with_unit("s")
                .with_order(11)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("nalpha")
                .with_display_name("1/f Exponent")
                .with_description("Flicker-noise exponent (0 < NALPHA < 2 when 1/f is enabled)")
                .with_type(PropertyType::Number)
                .with_default(PropertyValue::number(0.0))
                .with_order(12)
                .with_category("Noise"),
        );
        sheet.add(
            PropertyDefinition::new("namp")
                .with_display_name("1/f Amplitude")
                .with_description("Flicker-noise amplitude (0 disables the 1/f term)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(13)
                .with_category("Noise"),
        );
        // RTSAM/RTSCAPT/RTSEMT, the positional tail of the same card. The
        // engine synthesizes a real two-state telegraph from them
        // (`engine/transient/noise.rs` `add_rts_points`, lines 213-281): the
        // amplitude is the step height, and the two means are the dwell in the
        // low and high states respectively (`noise.rs:247-252`). They are an
        // all-or-nothing group — see `source_contract::trnoise_findings`.
        sheet.add(
            PropertyDefinition::new("rtsam")
                .with_display_name("RTS Amplitude")
                .with_description("Random-telegraph step height (0 disables the RTS term)")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(unit)
                .with_order(14)
                .with_category("Random telegraph"),
        );
        sheet.add(
            PropertyDefinition::new("rtscapt")
                .with_display_name("Capture Mean Time")
                .with_description("Mean dwell in the low state; must pair with the emission time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(15)
                .with_category("Random telegraph"),
        );
        sheet.add(
            PropertyDefinition::new("rtsemt")
                .with_display_name("Emission Mean Time")
                .with_description("Mean dwell in the high state; must pair with the capture time")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(16)
                .with_category("Random telegraph"),
        );

        Self::add_ac_params(&mut sheet, unit, 0.0);
        Self::add_advanced_ac_params(&mut sheet, unit);
        Self::add_distortion_params(&mut sheet, unit);
        Self::add_parasitics_params(&mut sheet, driven.has_series_resistance());
        // The transient-noise train is the whole point of the device, so its
        // switch sits at the end of the Noise group rather than in the shared
        // block, where it would land above NA.
        sheet.add(
            PropertyDefinition::new("isnoisy")
                .with_display_name("Noisy")
                .with_description("Clear to emit the DC value alone, with no noise train")
                .with_type(PropertyType::Boolean)
                .with_default(PropertyValue::boolean(true))
                .with_order(200)
                .with_category("Noise"),
        );
        sheet
    }

    /// File-backed PWL: `PWL FILE="path"` with the reader's scaling modifiers.
    ///
    /// The waveform itself lives in a CSV or WAV file rather than in the deck,
    /// so this sheet carries no point table — only the reference and the
    /// transforms applied to what the file contains. Every field here is one
    /// the netlist reader accepts, spelled the way it accepts it.
    pub(super) fn create_pwl_file_sheet(driven: Driven) -> PropertySheet {
        let mut sheet = PropertySheet::new();
        sheet.add(instance_name(driven));

        sheet.add(
            PropertyDefinition::new("file")
                .with_display_name("Data File")
                .with_description("CSV (time,value rows) or WAV file holding the waveform")
                .with_type(PropertyType::String)
                .with_default(PropertyValue::string(""))
                .with_order(10)
                .with_category("Data File")
                .required(),
        );
        sheet.add(
            PropertyDefinition::new("td")
                .with_display_name("Delay Time")
                .with_description(
                    "Time delay before the waveform starts; the source is zero until then",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(11)
                .with_category("Data File"),
        );
        sheet.add(
            PropertyDefinition::new("r")
                .with_display_name("Repeat From")
                .with_description(
                    "Source time the waveform repeats from after its last point; 0 repeats the whole file, blank never repeats",
                )
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::string(""))
                .with_unit("s")
                .with_order(12)
                .with_category("Data File"),
        );

        // Scaling is applied to the file's own columns as it is read. The
        // engine rejects a non-positive or non-finite TSCALE, so the range is
        // the reader's, not a guess.
        sheet.add(
            PropertyDefinition::new("tscale")
                .with_display_name("Time Scale")
                .with_description("Multiplies every time in the file")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(20)
                .with_category("Scaling"),
        );
        sheet.add(
            PropertyDefinition::new("vscale")
                .with_display_name("Value Scale")
                .with_description("Multiplies every value in the file")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(1.0))
                .with_order(21)
                .with_category("Scaling"),
        );
        sheet.add(
            PropertyDefinition::new("toffset")
                .with_display_name("Time Offset")
                .with_description("Added to every time after scaling")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit("s")
                .with_order(22)
                .with_category("Scaling"),
        );
        sheet.add(
            PropertyDefinition::new("voffset")
                .with_display_name("Value Offset")
                .with_description("Added to every value after scaling")
                .with_type(PropertyType::Expression)
                .with_default(PropertyValue::number(0.0))
                .with_unit(driven.unit())
                .with_order(23)
                .with_category("Scaling"),
        );

        Self::finish_source_sheet(&mut sheet, driven);
        sheet
    }

    /// The AC, advanced-AC, distortion, parasitic, and noise blocks every
    /// source carries.
    fn finish_source_sheet(sheet: &mut PropertySheet, driven: Driven) {
        Self::add_ac_params(sheet, driven.unit(), 0.0);
        Self::add_advanced_ac_params(sheet, driven.unit());
        Self::add_distortion_params(sheet, driven.unit());
        Self::add_parasitics_params(sheet, driven.has_series_resistance());
        Self::add_noise_params(sheet);
    }
}
