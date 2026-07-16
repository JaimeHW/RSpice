use std::collections::BTreeMap;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

trait WirePreference:
    Copy + Default + Eq + Serialize + DeserializeOwned + std::fmt::Debug + 'static
{
    const KEY: &'static str;
}

macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident for $key:literal {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident
            ),+ $(,)?
        }
        default $default:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
        }

        impl Default for $name {
            fn default() -> Self {
                Self::$default
            }
        }

        impl WirePreference for $name {
            const KEY: &'static str = $key;
        }
    };
}

wire_enum! {
    /// Preferred presentation family. Layout presentation is intentionally
    /// advisory and never changes a PDK database unit.
    pub enum UnitSystem for "unit-system" {
        MixedEngineering,
        Si,
        ImperialLayout
    }
    default MixedEngineering;
}

wire_enum! {
    /// Suffix policy for interactive UI fields. This is not [`SpiceDialect`]
    /// and does not affect imported or generated decks.
    pub enum EngineeringSuffixPolicy for "engineering-suffixes" {
        StrictRspice,
        ClassicSpiceCompatibility
    }
    default StrictRspice;
}

wire_enum! {
    /// Presentation of values stored in hertz.
    pub enum FrequencyDisplay for "frequency-display" {
        HertzEngineering,
        RadiansPerSecond
    }
    default HertzEngineering;
}

wire_enum! {
    /// Presentation of values stored in kelvin.
    pub enum TemperatureDisplay for "temperature-display" {
        Celsius,
        Kelvin,
        Fahrenheit
    }
    default Celsius;
}

wire_enum! {
    /// Clipboard representation. Both forms retain an explicit unit.
    pub enum CopiedValueFormat for "copied-value-format" {
        EngineeringNotationWithUnit,
        ScientificNotationWithSiUnit
    }
    default EngineeringNotationWithUnit;
}

wire_enum! {
    /// Presentation of values stored in radians.
    pub enum AngleDisplay for "angle-display" {
        Degrees,
        Radians
    }
    default Degrees;
}

wire_enum! {
    /// Layout-coordinate presentation only. A formatter must be supplied the
    /// authoritative PDK DBU before this value can be applied.
    pub enum LayoutCoordinateDisplay for "layout-coordinate-display" {
        MicrometresWithDatabaseUnitRemainder,
        Nanometres,
        DatabaseUnits
    }
    default MicrometresWithDatabaseUnitRemainder;
}

wire_enum! {
    /// Whether bare time/frequency numbers are accepted in typed UI fields.
    pub enum TimeFrequencyInput for "time-frequency-input" {
        StrictUnitsRequired,
        InferFromFieldQuantity
    }
    default StrictUnitsRequired;
}

wire_enum! {
    /// Decimal-mark behavior for interactive fields. Portable files always
    /// use a period and do not consume this preference.
    pub enum DecimalSeparatorInput for "decimal-separator-input" {
        LocaleAwareUiPortableFiles,
        PeriodEverywhere
    }
    default LocaleAwareUiPortableFiles;
}

/// Forward-compatible wire domain for user unit preferences.
///
/// Known values have typed accessors. Unknown keys and future enum spellings
/// remain byte-semantically represented as JSON values so a newer profile can
/// pass through an older build without invalidating the recoverable session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UnitsPreferences {
    values: BTreeMap<String, Value>,
}

impl UnitsPreferences {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn get<T: WirePreference>(&self) -> T {
        self.values
            .get(T::KEY)
            .cloned()
            .and_then(|value| serde_json::from_value(value).ok())
            .unwrap_or_default()
    }

    fn set<T: WirePreference>(&mut self, value: T) {
        if value == T::default() {
            self.values.remove(T::KEY);
        } else {
            self.values.insert(
                T::KEY.to_owned(),
                serde_json::to_value(value).expect("unit preference enums always serialize"),
            );
        }
    }

    #[must_use]
    pub fn unit_system(&self) -> UnitSystem {
        self.get()
    }

    pub fn set_unit_system(&mut self, value: UnitSystem) {
        self.set(value);
    }

    #[must_use]
    pub fn engineering_suffixes(&self) -> EngineeringSuffixPolicy {
        self.get()
    }

    pub fn set_engineering_suffixes(&mut self, value: EngineeringSuffixPolicy) {
        self.set(value);
    }

    #[must_use]
    pub fn frequency_display(&self) -> FrequencyDisplay {
        self.get()
    }

    pub fn set_frequency_display(&mut self, value: FrequencyDisplay) {
        self.set(value);
    }

    #[must_use]
    pub fn temperature_display(&self) -> TemperatureDisplay {
        self.get()
    }

    pub fn set_temperature_display(&mut self, value: TemperatureDisplay) {
        self.set(value);
    }

    #[must_use]
    pub fn copied_value_format(&self) -> CopiedValueFormat {
        self.get()
    }

    pub fn set_copied_value_format(&mut self, value: CopiedValueFormat) {
        self.set(value);
    }

    #[must_use]
    pub fn angle_display(&self) -> AngleDisplay {
        self.get()
    }

    pub fn set_angle_display(&mut self, value: AngleDisplay) {
        self.set(value);
    }

    #[must_use]
    pub fn layout_coordinate_display(&self) -> LayoutCoordinateDisplay {
        self.get()
    }

    pub fn set_layout_coordinate_display(&mut self, value: LayoutCoordinateDisplay) {
        self.set(value);
    }

    #[must_use]
    pub fn time_frequency_input(&self) -> TimeFrequencyInput {
        self.get()
    }

    pub fn set_time_frequency_input(&mut self, value: TimeFrequencyInput) {
        self.set(value);
    }

    #[must_use]
    pub fn decimal_separator_input(&self) -> DecimalSeparatorInput {
        self.get()
    }

    pub fn set_decimal_separator_input(&mut self, value: DecimalSeparatorInput) {
        self.set(value);
    }

    /// Immutable policy snapshot suitable for one render, copy, or input
    /// transaction. Later preference changes cannot alter an in-flight use.
    #[must_use]
    pub fn presentation_policy(&self) -> QuantityPresentationPolicy {
        QuantityPresentationPolicy {
            unit_system: self.unit_system(),
            engineering_suffixes: self.engineering_suffixes(),
            frequency_display: self.frequency_display(),
            temperature_display: self.temperature_display(),
            copied_value_format: self.copied_value_format(),
            angle_display: self.angle_display(),
            layout_coordinate_display: self.layout_coordinate_display(),
            time_frequency_input: self.time_frequency_input(),
            decimal_separator_input: self.decimal_separator_input(),
        }
    }

    pub(crate) fn contains_wire_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    /// Transitional adapter for the existing Preferences renderer. Durable
    /// storage and runtime consumers remain typed; the renderer's ordinal is
    /// converted at this boundary only.
    pub(crate) fn compatibility_index(&self, key: &str) -> Option<usize> {
        Some(match key {
            UnitSystem::KEY => match self.unit_system() {
                UnitSystem::MixedEngineering => 0,
                UnitSystem::Si => 1,
                UnitSystem::ImperialLayout => 2,
            },
            EngineeringSuffixPolicy::KEY => match self.engineering_suffixes() {
                EngineeringSuffixPolicy::StrictRspice => 0,
                EngineeringSuffixPolicy::ClassicSpiceCompatibility => 1,
            },
            FrequencyDisplay::KEY => match self.frequency_display() {
                FrequencyDisplay::HertzEngineering => 0,
                FrequencyDisplay::RadiansPerSecond => 1,
            },
            TemperatureDisplay::KEY => match self.temperature_display() {
                TemperatureDisplay::Celsius => 0,
                TemperatureDisplay::Kelvin => 1,
                TemperatureDisplay::Fahrenheit => 2,
            },
            CopiedValueFormat::KEY => match self.copied_value_format() {
                CopiedValueFormat::EngineeringNotationWithUnit => 0,
                CopiedValueFormat::ScientificNotationWithSiUnit => 1,
            },
            AngleDisplay::KEY => match self.angle_display() {
                AngleDisplay::Degrees => 0,
                AngleDisplay::Radians => 1,
            },
            LayoutCoordinateDisplay::KEY => match self.layout_coordinate_display() {
                LayoutCoordinateDisplay::MicrometresWithDatabaseUnitRemainder => 0,
                LayoutCoordinateDisplay::Nanometres => 1,
                LayoutCoordinateDisplay::DatabaseUnits => 2,
            },
            TimeFrequencyInput::KEY => match self.time_frequency_input() {
                TimeFrequencyInput::StrictUnitsRequired => 0,
                TimeFrequencyInput::InferFromFieldQuantity => 1,
            },
            DecimalSeparatorInput::KEY => match self.decimal_separator_input() {
                DecimalSeparatorInput::LocaleAwareUiPortableFiles => 0,
                DecimalSeparatorInput::PeriodEverywhere => 1,
            },
            _ => return None,
        })
    }

    pub(crate) fn set_compatibility_index(
        &mut self,
        key: &str,
        index: usize,
    ) -> Option<Result<(), &'static str>> {
        match key {
            UnitSystem::KEY => match index {
                0 => self.set_unit_system(UnitSystem::MixedEngineering),
                1 => self.set_unit_system(UnitSystem::Si),
                2 => self.set_unit_system(UnitSystem::ImperialLayout),
                _ => return Some(Err("unit system index is outside its domain")),
            },
            EngineeringSuffixPolicy::KEY => match index {
                0 => self.set_engineering_suffixes(EngineeringSuffixPolicy::StrictRspice),
                1 => self
                    .set_engineering_suffixes(EngineeringSuffixPolicy::ClassicSpiceCompatibility),
                _ => return Some(Err("engineering suffix index is outside its domain")),
            },
            FrequencyDisplay::KEY => match index {
                0 => self.set_frequency_display(FrequencyDisplay::HertzEngineering),
                1 => self.set_frequency_display(FrequencyDisplay::RadiansPerSecond),
                _ => return Some(Err("frequency display index is outside its domain")),
            },
            TemperatureDisplay::KEY => match index {
                0 => self.set_temperature_display(TemperatureDisplay::Celsius),
                1 => self.set_temperature_display(TemperatureDisplay::Kelvin),
                2 => self.set_temperature_display(TemperatureDisplay::Fahrenheit),
                _ => return Some(Err("temperature display index is outside its domain")),
            },
            CopiedValueFormat::KEY => match index {
                0 => self.set_copied_value_format(CopiedValueFormat::EngineeringNotationWithUnit),
                1 => self.set_copied_value_format(CopiedValueFormat::ScientificNotationWithSiUnit),
                _ => return Some(Err("copied value format index is outside its domain")),
            },
            AngleDisplay::KEY => match index {
                0 => self.set_angle_display(AngleDisplay::Degrees),
                1 => self.set_angle_display(AngleDisplay::Radians),
                _ => return Some(Err("angle display index is outside its domain")),
            },
            LayoutCoordinateDisplay::KEY => match index {
                0 => self.set_layout_coordinate_display(
                    LayoutCoordinateDisplay::MicrometresWithDatabaseUnitRemainder,
                ),
                1 => self.set_layout_coordinate_display(LayoutCoordinateDisplay::Nanometres),
                2 => self.set_layout_coordinate_display(LayoutCoordinateDisplay::DatabaseUnits),
                _ => return Some(Err("layout coordinate display index is outside its domain")),
            },
            TimeFrequencyInput::KEY => match index {
                0 => self.set_time_frequency_input(TimeFrequencyInput::StrictUnitsRequired),
                1 => self.set_time_frequency_input(TimeFrequencyInput::InferFromFieldQuantity),
                _ => return Some(Err("time/frequency input index is outside its domain")),
            },
            DecimalSeparatorInput::KEY => match index {
                0 => self
                    .set_decimal_separator_input(DecimalSeparatorInput::LocaleAwareUiPortableFiles),
                1 => self.set_decimal_separator_input(DecimalSeparatorInput::PeriodEverywhere),
                _ => return Some(Err("decimal separator index is outside its domain")),
            },
            _ => return None,
        };
        Some(Ok(()))
    }
}

/// Resolved, immutable UI-edge unit policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuantityPresentationPolicy {
    pub unit_system: UnitSystem,
    pub engineering_suffixes: EngineeringSuffixPolicy,
    pub frequency_display: FrequencyDisplay,
    pub temperature_display: TemperatureDisplay,
    pub copied_value_format: CopiedValueFormat,
    pub angle_display: AngleDisplay,
    pub layout_coordinate_display: LayoutCoordinateDisplay,
    pub time_frequency_input: TimeFrequencyInput,
    pub decimal_separator_input: DecimalSeparatorInput,
}

impl Default for QuantityPresentationPolicy {
    fn default() -> Self {
        UnitsPreferences::default().presentation_policy()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_domain_omits_defaults_and_round_trips_overrides() {
        let mut preferences = UnitsPreferences::default();
        preferences.set_frequency_display(FrequencyDisplay::RadiansPerSecond);
        preferences.set_temperature_display(TemperatureDisplay::Kelvin);
        preferences.set_angle_display(AngleDisplay::Radians);

        let value = serde_json::to_value(&preferences).unwrap();
        assert_eq!(value["frequency-display"], "radians-per-second");
        assert_eq!(value["temperature-display"], "kelvin");
        assert_eq!(value["angle-display"], "radians");
        assert!(value.get("unit-system").is_none());

        let restored: UnitsPreferences = serde_json::from_value(value).unwrap();
        assert_eq!(
            restored.frequency_display(),
            FrequencyDisplay::RadiansPerSecond
        );
        assert_eq!(restored.temperature_display(), TemperatureDisplay::Kelvin);
        assert_eq!(restored.angle_display(), AngleDisplay::Radians);
    }

    #[test]
    fn future_keys_and_known_key_variants_survive_an_older_reader() {
        let source = serde_json::json!({
            "frequency-display": "future-quantum-frequency",
            "future-exact-copy": {"digits": 19}
        });
        let preferences: UnitsPreferences = serde_json::from_value(source.clone()).unwrap();

        assert_eq!(
            preferences.frequency_display(),
            FrequencyDisplay::HertzEngineering
        );
        assert_eq!(serde_json::to_value(preferences).unwrap(), source);
    }
}
