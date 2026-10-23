//! The HDL signals a run traces by name: the nets and variables of an
//! instance's design that `.ESAVE` lists (`.esave xtb.pass xtb.mv`), which
//! the run publishes through the observer the value change dump uses.

use rspice_veriloga::canonical_ir::digital::DigitalSignal;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;

use super::MixedSignalHost;

/// A net or variable of an instance's design that has a value to trace.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct HdlVariable {
    pub signal: DigitalSignalId,
    /// Its name as declared.
    pub identifier: String,
    pub real: bool,
    /// `[msb:lsb]` as declared, for a vector; a scalar and a real have none.
    pub range: Option<(i64, i64)>,
}

impl HdlVariable {
    /// How many trace slots it publishes: one per bit, or one real.
    pub(crate) fn slots(&self) -> usize {
        match self.range {
            Some((msb, lsb)) => {
                usize::try_from(msb.abs_diff(lsb)).map_or(usize::MAX, |span| span.saturating_add(1))
            }
            None => 1,
        }
    }
}

/// Whether a signal holds a value: a named event has none.
fn valued(signal: &DigitalSignal) -> bool {
    signal.kind.is_real() || signal.width > 0
}

impl MixedSignalHost {
    /// The net or variable `identifier` names in this instance's design: the
    /// one declared with exactly that name, or else the only one whose name
    /// matches it ignoring case, as a deck that folds case spells it.
    pub(crate) fn hdl_variable(&self, identifier: &str) -> Option<HdlVariable> {
        let plan = self.digital_plan();
        let candidates = || plan.signals.iter().filter(|signal| valued(signal));
        let signal = match candidates().find(|signal| signal.name == identifier) {
            Some(signal) => signal,
            None => {
                let mut folded =
                    candidates().filter(|signal| signal.name.eq_ignore_ascii_case(identifier));
                let only = folded.next()?;
                if folded.next().is_some() {
                    return None;
                }
                only
            }
        };
        let real = signal.kind.is_real();
        let range = if real {
            None
        } else {
            signal
                .bounds
                .or_else(|| (signal.width > 1).then(|| (i64::from(signal.width) - 1, 0)))
        };
        Some(HdlVariable {
            signal: signal.id,
            identifier: signal.name.to_string(),
            real,
            range,
        })
    }
}
