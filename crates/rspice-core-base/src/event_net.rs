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
//! `analysis::transient` re-exports all three, which is where every
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
    /// (`execution::transient_projection_signals`) and what the
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

/// What one module port carries across the boundary between the analog solver
/// and the event kernel.
///
/// A port its module's discrete plan names is an event endpoint: four-state
/// logic, one net per bit, or a real value on one net. Any other port is an
/// electrical terminal. The engine lays out an instance card by this, and a
/// design states its pins by the same answer, so the two cannot disagree about
/// which conductors a port has or what they carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PortCarrier {
    /// A continuous terminal: one electrical node.
    Electrical,
    /// Four-state logic with drive strength: one event net per bit.
    Logic,
    /// A real value: one event net.
    Real,
}

impl PortCarrier {
    /// The word a pin or a net is described by.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Electrical => "electrical",
            Self::Logic => "logic",
            Self::Real => "real",
        }
    }
}

impl PortCarrier {
    /// The carrier a label from [`PortCarrier::label`] names.
    pub fn from_label(label: &str) -> Option<Self> {
        [Self::Electrical, Self::Logic, Self::Real]
            .into_iter()
            .find(|carrier| carrier.label() == label)
    }
}

/// Whether the event endpoints on one net join directly on the event kernel,
/// with no converter between them and the analog solver.
///
/// They do only when nothing else is there: the net is not ground, no
/// electrical terminal is on it, and at least two event endpoints meet. One
/// module port alone on a net is an analog node its connect module drives,
/// which is what Spectre and AMS Designer show, so it keeps its converter.
/// The engine plans its converters by this, and a design states each net's
/// kind by it before anything has run.
pub const fn joins_on_event_kernel(ground: bool, electrical: bool, event_endpoints: usize) -> bool {
    !ground && !electrical && event_endpoints >= 2
}

/// What meets on one net, counted by what each conductor carries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NetEndpoints {
    /// The net is the ground node.
    pub ground: bool,
    /// Electrical terminals on the net.
    pub electrical: usize,
    /// Four-state logic endpoints on the net.
    pub logic: usize,
    /// Real-valued endpoints on the net.
    pub real: usize,
    /// How many of the logic and real endpoints are module ports. The rest
    /// are code-model event ports, which a net does not need a second
    /// endpoint to keep.
    pub module_ports: usize,
}

impl NetEndpoints {
    /// Count one conductor; `module_port` says an event endpoint is a module
    /// port rather than a code model's.
    pub fn add(&mut self, carrier: PortCarrier, module_port: bool) {
        match carrier {
            PortCarrier::Electrical => self.electrical += 1,
            PortCarrier::Logic => self.logic += 1,
            PortCarrier::Real => self.real += 1,
        }
        if module_port && carrier != PortCarrier::Electrical {
            self.module_ports += 1;
        }
    }

    /// What the net carries at run, by the rule the engine plans with.
    pub const fn domain(&self) -> NetDomain {
        let event = self.logic + self.real;
        if event == 0 {
            return NetDomain::Electrical;
        }
        let analog = self.ground || self.electrical > 0;
        let converts = if self.module_ports > 0 {
            !joins_on_event_kernel(self.ground, self.electrical > 0, event)
        } else {
            analog
        };
        if converts {
            NetDomain::Mixed
        } else if self.logic > 0 && self.real > 0 {
            NetDomain::LogicAndReal
        } else if self.logic > 0 {
            NetDomain::Logic
        } else {
            NetDomain::Real
        }
    }
}

/// What one net carries at run.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum NetDomain {
    /// Only electrical terminals: one analog node.
    #[default]
    Electrical,
    /// Four-state logic resolved on the event kernel.
    Logic,
    /// A real value resolved on the event kernel.
    Real,
    /// Event endpoints meet the analog solver through a converter the engine
    /// inserts when the run is prepared.
    Mixed,
    /// Logic and real ports on one event net, which the engine refuses: the
    /// two meet only through an explicit conversion.
    LogicAndReal,
}

impl NetDomain {
    /// The words a net's kind is shown in.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Electrical => "electrical",
            Self::Logic => "logic",
            Self::Real => "real",
            Self::Mixed => "mixed (converter at run)",
            Self::LogicAndReal => "logic and real (refused at run)",
        }
    }
}

#[cfg(test)]
mod net_domain_tests {
    use super::{NetDomain, NetEndpoints, PortCarrier};

    fn net(ground: bool, carriers: &[(PortCarrier, bool)]) -> NetDomain {
        let mut endpoints = NetEndpoints {
            ground,
            ..NetEndpoints::default()
        };
        for &(carrier, module_port) in carriers {
            endpoints.add(carrier, module_port);
        }
        endpoints.domain()
    }

    #[test]
    fn a_net_reads_as_the_engine_plans_it() {
        use PortCarrier::{Electrical, Logic, Real};
        assert_eq!(
            net(false, &[(Electrical, false), (Electrical, true)]),
            NetDomain::Electrical
        );
        assert_eq!(
            net(false, &[(Logic, true), (Logic, true)]),
            NetDomain::Logic
        );
        assert_eq!(net(false, &[(Real, true), (Real, false)]), NetDomain::Real);
        assert_eq!(
            net(false, &[(Logic, true), (Electrical, false)]),
            NetDomain::Mixed
        );
        assert_eq!(net(true, &[(Logic, true), (Logic, true)]), NetDomain::Mixed);
        // One module port alone is an analog node its connect module drives;
        // one code-model port alone is an event net.
        assert_eq!(net(false, &[(Logic, true)]), NetDomain::Mixed);
        assert_eq!(net(false, &[(Logic, false)]), NetDomain::Logic);
        assert_eq!(
            net(false, &[(Logic, true), (Real, true)]),
            NetDomain::LogicAndReal
        );
        for carrier in [Electrical, Logic, Real] {
            assert_eq!(PortCarrier::from_label(carrier.label()), Some(carrier));
        }
    }
}
