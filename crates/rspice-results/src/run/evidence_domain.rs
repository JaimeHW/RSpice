//! Retained dataset ownership for plan-limit evidence.

use super::SimulationRun;
use rspice_app_types::product::SimulationPlanId;

/// How the selected dataset relates to the plan whose limits are being read.
///
/// Only [`Self::ThisPlan`] and [`Self::Legacy`] are datasets a plan's limits may
/// be judged against. The rest are not refusals to show anything — they are the
/// reason a surface has nothing to show, and naming that reason is the whole
/// point: "no evidence" and "the evidence belongs to another plan" send an
/// engineer to two different places.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceDomain {
    /// The plan being read owns the selected run.
    ThisPlan,
    /// Another plan owns it.
    AnotherPlan,
    /// A manual deck produced it, so no plan owns it.
    ManualDeck,
    /// History from before runs recorded plan ownership. Readable, because
    /// refusing it would hide every result a project had before receipts.
    Legacy,
    /// Nothing is selected.
    NoDataset,
}

impl EvidenceDomain {
    /// Whether a plan's limits may be answered by this dataset.
    #[must_use]
    pub const fn answers_a_plan_limit(self) -> bool {
        matches!(self, Self::ThisPlan | Self::Legacy)
    }

    /// What a surface says when the dataset cannot answer its limits.
    #[must_use]
    pub const fn refusal(self) -> Option<&'static str> {
        match self {
            Self::ThisPlan | Self::Legacy => None,
            Self::AnotherPlan => Some("active dataset belongs to another plan"),
            Self::ManualDeck => Some("active dataset is a manual deck, owned by no plan"),
            Self::NoDataset => Some("no dataset loaded"),
        }
    }
}

impl<A> SimulationRun<A> {
    /// Classify this retained dataset against the plan whose limits are read.
    #[must_use]
    pub fn evidence_domain(&self, plan_id: Option<SimulationPlanId>) -> EvidenceDomain {
        let Some(receipt) = self.prepared_receipt() else {
            return EvidenceDomain::Legacy;
        };
        match (receipt.simulation_plan_id(), plan_id) {
            (Some(owner), Some(reader)) if owner == reader => EvidenceDomain::ThisPlan,
            (Some(_), _) => EvidenceDomain::AnotherPlan,
            (None, _) => EvidenceDomain::ManualDeck,
        }
    }
}
