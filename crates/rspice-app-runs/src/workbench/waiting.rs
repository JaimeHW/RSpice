//! A manual deck's validation, or a campaign's start, that waits for a
//! Verilog-A compile instead of holding the frame.
//!
//! Both are prepared in the frame that takes the press. A PDK's or a model
//! library's Verilog-A source compiles on a background thread once a frame
//! names it, and a preparation that reached a source still compiling waited
//! for that compile in the frame: the window froze for as long as the compile
//! took, 9.7 s for a BSIM-BULK PDK in a debug build. Such a request now waits
//! beside the frame instead. The status bar says what waits, the frame loop
//! carries it out once the sources its preparations read are compiled, and
//! Stop drops it.

use crate::product::SimulationPlanId;

/// What waits for a Verilog-A compile to end.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WaitingPreparations {
    /// The manual deck's validation.
    pub validation: Option<WaitingValidation>,
    /// A campaign's start.
    pub campaign: Option<WaitingCampaign>,
}

/// A manual deck's validation that waits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaitingValidation {
    /// The format gesture asked for it: the deck is formatted once it
    /// validates.
    pub then_format: bool,
}

/// A campaign's start that waits: what the campaign dialog committed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaitingCampaign {
    pub name: String,
    pub member_ids: Vec<SimulationPlanId>,
}

impl WaitingPreparations {
    /// Whether nothing waits.
    pub fn is_empty(&self) -> bool {
        self.validation.is_none() && self.campaign.is_none()
    }

    /// Drop everything that waits. Returns what was dropped, each as the
    /// reader would name it, for the log line that says so.
    pub fn cancel(&mut self) -> Vec<String> {
        let validation = self
            .validation
            .take()
            .map(|_| "the netlist's validation".to_owned());
        let campaign = self
            .campaign
            .take()
            .map(|campaign| format!("campaign '{}'", campaign.name));
        validation.into_iter().chain(campaign).collect()
    }

    /// What the status bar says while something waits.
    pub fn status(&self) -> Option<String> {
        let what = match (&self.validation, &self.campaign) {
            (None, None) => return None,
            (Some(_), None) => "Validating the netlist".to_owned(),
            (None, Some(campaign)) => format!("Campaign {}", campaign.name),
            (Some(_), Some(campaign)) => {
                format!("Validating the netlist and campaign {}", campaign.name)
            }
        };
        Some(format!("{what} \u{b7} waiting for the Verilog-A compile"))
    }
}

/// Whether a Verilog-A source the frame's preparations read is compiling on
/// another thread, so a preparation started now would wait for it.
pub fn compiling() -> bool {
    #[cfg(any(test, feature = "test-support"))]
    if test_hold::held() {
        return true;
    }
    rspice_simulation::project_veriloga::external_runtimes::compiling()
}

/// `prepare`'s result, or `None` when it reached a Verilog-A source another
/// thread is compiling: it stops there instead of waiting, and [`compiling`]
/// holds until that compile ends.
pub fn unless_compiling<R>(prepare: impl FnOnce() -> R) -> Option<R> {
    rspice_simulation::project_veriloga::external_runtimes::unless_compiling(prepare)
}

/// Hold [`compiling`] on this thread until the returned guard drops, as a
/// compile in flight would.
#[cfg(any(test, feature = "test-support"))]
pub fn hold_compiling() -> CompileHold {
    test_hold::hold()
}

#[cfg(any(test, feature = "test-support"))]
pub use test_hold::CompileHold;

#[cfg(any(test, feature = "test-support"))]
mod test_hold {
    use std::cell::Cell;

    thread_local! {
        static HOLDS: Cell<usize> = const { Cell::new(0) };
    }

    /// While it lives, [`super::compiling`] holds on the thread that took it.
    pub struct CompileHold(());

    impl Drop for CompileHold {
        fn drop(&mut self) {
            HOLDS.with(|holds| holds.set(holds.get().saturating_sub(1)));
        }
    }

    pub(super) fn hold() -> CompileHold {
        HOLDS.with(|holds| holds.set(holds.get() + 1));
        CompileHold(())
    }

    pub(super) fn held() -> bool {
        HOLDS.with(|holds| holds.get() > 0)
    }
}
