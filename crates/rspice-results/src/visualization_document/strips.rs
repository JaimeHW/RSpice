//! How the reader arranged a waveform pane into strips.
//!
//! A strip is one plot row of a waveform pane: it shares the pane's X axis
//! and holds one to four Y axes, each reading in one unit, as ViVA's strips
//! do. A pane the reader never arranged keeps no strips, and its stack gives
//! each unit a strip of its own; the first arrangement writes every strip
//! down. A strip names what it draws by signal and form, or by expression
//! text, so a view that follows a new run keeps its strips by name.

use super::*;

/// The most Y axes one strip holds: one per unit, as ViVA allows.
pub const MAX_STRIP_AXES: usize = 4;

/// Which projection of a signal a strip member draws: an AC signal is drawn
/// as its magnitude and its phase, each on an axis of its own unit.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignalForm {
    /// The values as solved: a voltage, a current, a sweep output.
    #[default]
    Value,
    /// An AC magnitude in decibels.
    Magnitude,
    /// An AC phase.
    Phase,
    /// The real part of a complex value.
    Real,
    /// The imaginary part of a complex value.
    Imaginary,
    /// A noise power density read as an amplitude density.
    NoiseDensity,
    /// The group delay of a complex response, in seconds.
    GroupDelay,
    /// A noise power density read as the power density it is.
    NoisePower,
}

impl SignalForm {
    fn is_value(&self) -> bool {
        *self == Self::Value
    }
}

/// One curve a strip axis draws.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum StripMember {
    /// A signal of the pane's analysis, by the name its run gives it.
    Signal {
        #[serde(deserialize_with = "deserialize_label_string")]
        name: String,
        #[serde(default, skip_serializing_if = "SignalForm::is_value")]
        form: SignalForm,
    },
    /// A calculator expression the pane plots, by its text.
    Expression {
        #[serde(deserialize_with = "deserialize_source_text_string")]
        text: String,
    },
}

/// One Y axis of a strip: its unit and the curves read against it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StripAxis {
    /// The unit the axis reads in, as the stack names it: empty for a plain
    /// number. A curve of another unit is never drawn against it.
    #[serde(deserialize_with = "deserialize_label_string")]
    pub unit: String,
    pub members: Vec<StripMember>,
}

/// One strip of a waveform pane, top to bottom in the pane's list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Strip {
    /// The strip's identity in its pane, which its zoom is kept by.
    #[serde(deserialize_with = "deserialize_key_string")]
    pub key: String,
    /// Its share of the stack's height against the others' (1 each when
    /// none is set). Absent, an equal share.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f32>,
    /// The reader locked its height: resizing its neighbours and equalising
    /// the stack leave it as it is.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub locked: bool,
    /// Its Y axes, in the order their gutters stand.
    pub axes: Vec<StripAxis>,
}

impl Strip {
    /// Every curve the strip draws, axis by axis.
    pub fn members(&self) -> impl Iterator<Item = &StripMember> {
        self.axes.iter().flat_map(|axis| axis.members.iter())
    }
}

/// Why a pane's strips cannot stand as given.
fn invalid(message: impl Into<String>) -> VisualizationError {
    VisualizationError::InvalidValue {
        field: "pane.strips",
        message: message.into(),
    }
}

/// Check one pane's strips: each has a key of its own, one to four axes and
/// a share of the height above zero, and draws no curve twice. A curve may
/// stand on two strips: one copied to a new strip is drawn on both.
pub(super) fn validate_strips(strips: &[Strip]) -> Result<(), VisualizationError> {
    let mut keys = HashSet::new();
    for strip in strips {
        validate_key("pane.strips.key", &strip.key)?;
        if !keys.insert(strip.key.as_str()) {
            return Err(invalid(format!("two strips share the key {:?}", strip.key)));
        }
        if strip.axes.is_empty() {
            return Err(invalid(format!("strip {:?} has no Y axis", strip.key)));
        }
        if strip.axes.len() > MAX_STRIP_AXES {
            return Err(invalid(format!(
                "a strip holds {MAX_STRIP_AXES} Y axes at most; strip {:?} has {}",
                strip.key,
                strip.axes.len()
            )));
        }
        if let Some(weight) = strip.weight
            && !(weight.is_finite() && weight > 0.0)
        {
            return Err(invalid(format!(
                "strip {:?} takes a share of the height above zero",
                strip.key
            )));
        }
        let mut members = HashSet::new();
        for member in strip.members() {
            if !members.insert(member) {
                return Err(invalid(format!(
                    "strip {:?} draws {member:?} twice",
                    strip.key
                )));
            }
        }
    }
    Ok(())
}
