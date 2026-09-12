//! The vocabulary an event-only net is refused and published under.
//!
//! A net that only the event domain resolves owns an MNA row the assembly
//! closes with `v = 0` to restore rank, so every surface that could hand that
//! placeholder back as a voltage refuses instead -- and each has to name the
//! same carrier in the same sentence, or a reader is sent to an accessor the
//! object in their hand does not have.
//!
//! This is a leaf for the reason `codemodels` and `op_label` are: the names
//! are data. A solved point in `solver` and the analog-touch classification in
//! `circuit` both need them, and both sit far below `analysis`, where the two
//! enums used to live -- so naming them there was the solver reaching seven
//! ranks up the architecture for two field-less enums and a `format!`.
//! [`crate::analysis::transient`] re-exports all three, which is where every
//! frontend still spells them.

/// Which event domain resolves a net that the analog system does not.
///
/// The two domains publish their values under different spellings, so the
/// refusal has to know which one it is naming: recommending a carrier that
/// does not carry this net is the same defect as publishing a placeholder
/// zero. A net that registered both domains is named as digital, because that
/// is the domain whose column an exported table carries for it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventOnlyNetKind {
    /// Four-state logic with drive strength.
    Digital,
    /// Real-valued event data.
    Real,
}

/// Which result object a refusal is being rendered for.
///
/// The two Python result classes spell the digital accessor differently —
/// `TransientResult.digital_events` against
/// `CompressedTransientResult.digital_trace` — so a sentence that names one of
/// them on the other recommends a method that class does not have, which is
/// the defect this whole refusal exists to stop making. The surface travels
/// with the refusal rather than the wording being duplicated per binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTraceSurface {
    /// The engine's own result, and the `TransientResult` binding over it.
    /// This is what a deck-side refusal names: a run has no decimated
    /// container until a caller asks for one.
    Result,
    /// The decimated `CompressedTransientResult` binding.
    Compressed,
    /// One solved DC point — an operating point, or a point of a `.DC`
    /// sweep — and the `SimulationResult` binding over it.
    ///
    /// This class carries no event trace of its own: a DC solve publishes one
    /// number per analog unknown and nothing else, so the accessor the
    /// refusal recommends is a transient run's. Naming this class's own
    /// accessor would be the defect the surface exists to stop making, in the
    /// other direction: recommending a method that does not exist at all.
    SolvedPoint,
}

impl EventOnlyNetKind {
    /// The column label an exported table gives this net's event trace.
    ///
    /// `D(..)` for digital is what the flattened projection writes
    /// ([`crate::execution::transient_projection_signals`]) and what the
    /// rawfile event plot declares; `E(..)` for real is the rawfile event
    /// plot's spelling, which the workbench event sheet reads back. Neither
    /// depends on which result object was asked.
    fn column_label(self, name: &str) -> String {
        match self {
            Self::Digital => format!("D({name})"),
            Self::Real => format!("E({name})"),
        }
    }

    /// The Python accessor that returns this net's event trace, as the class
    /// being addressed spells it.
    ///
    /// Only the digital accessor differs between the two transient classes;
    /// both spell the real one `real_trace`. A solved DC point publishes no
    /// event trace at all, so it names the transient class's accessor and says
    /// whose it is — a reader who is told to call a method has to be able to
    /// find the object that has it.
    fn python_accessor(self, name: &str, surface: EventTraceSurface) -> String {
        match (self, surface) {
            (Self::Digital, EventTraceSurface::Result) => format!("digital_events('{name}')"),
            (Self::Digital, EventTraceSurface::Compressed) => format!("digital_trace('{name}')"),
            (Self::Digital, EventTraceSurface::SolvedPoint) => {
                format!("a transient run's digital_events('{name}')")
            }
            (Self::Real, EventTraceSurface::SolvedPoint) => {
                format!("a transient run's real_trace('{name}')")
            }
            (Self::Real, _) => format!("real_trace('{name}')"),
        }
    }
}

/// The one sentence every surface renders when a voltage of an event-only net
/// is asked for.
///
/// A net that only the event domain resolves has no `V()`: the MNA row it owns
/// is a placeholder the assembly closes with `v = 0` to restore rank, so
/// publishing it as a voltage publishes 0 V for a net that carries events. The
/// carriers that do exist are named in the sentence, so a reader never has to
/// guess which spelling replaces the one that was refused — and each is named
/// as what it is, a column an export publishes and an accessor a binding
/// answers, rather than as another operand to write on a card. One function
/// because the namespace build, the post-run resolvers and the bindings all
/// have to say the same thing — `surface` picks which class's accessor is
/// named, not which sentence is said.
pub fn event_only_voltage_refusal(
    name: &str,
    kind: EventOnlyNetKind,
    surface: EventTraceSurface,
) -> String {
    let label = kind.column_label(name);
    let accessor = kind.python_accessor(name, surface);
    format!(
        "V({name}) names '{name}', an event-only net: it carries event values, not a voltage. \
         Its values are published as its event trace ({label}; {accessor}), not as an \
         output-card operand."
    )
}
