//! Retained preflight authorization and single-use dispatch.

use super::permit::{ExecutionPermit, ExecutionPermitIssuer};
use super::{AuthorizedRunDispatch, PreparationError, PreparationStage, PreparedRunSnapshot};
use crate::state::SimulationRunIntent;

struct PendingPreparedRun {
    snapshot: PreparedRunSnapshot,
    permit: ExecutionPermit,
}

/// Owns both the retained snapshot and the authority that can consume it.
/// Preparation stays with the owning workflow; this boundary compares its
/// freshly rebuilt snapshot before releasing a dispatch.
#[derive(Default)]
pub(in crate::simulation) struct PreparedRunAuthorization {
    pending: Option<PendingPreparedRun>,
    permits: ExecutionPermitIssuer,
}

impl PreparedRunAuthorization {
    pub(in crate::simulation) fn clear(&mut self) {
        self.pending = None;
        if let Err(error) = self.permits.invalidate() {
            log::error!("Failed to invalidate prepared execution permit: {error}");
        }
    }

    pub(in crate::simulation) fn retained_snapshot(&self) -> Option<&PreparedRunSnapshot> {
        self.pending.as_ref().map(|pending| &pending.snapshot)
    }

    pub(in crate::simulation) fn retain(
        &mut self,
        snapshot: PreparedRunSnapshot,
    ) -> Result<(), PreparationError> {
        let permit = self.permits.issue(snapshot.digest()).map_err(|error| {
            PreparationError::new(
                PreparationStage::Authorization,
                format!("Could not authorize prepared run: {error}"),
            )
        })?;
        self.pending = Some(PendingPreparedRun { snapshot, permit });
        Ok(())
    }

    pub(in crate::simulation) fn consume(
        &mut self,
        intent: SimulationRunIntent,
        rebuild: impl FnOnce() -> Result<PreparedRunSnapshot, PreparationError>,
    ) -> Result<AuthorizedRunDispatch, PreparationError> {
        if self
            .pending
            .as_ref()
            .is_some_and(|pending| pending.snapshot.intent() != intent)
        {
            self.clear();
        }

        if self.pending.is_none() {
            let message = match intent {
                SimulationRunIntent::ManualDeck => {
                    "Validate the exact current netlist before running; manual decks are never auto-authorized"
                }
                SimulationRunIntent::SimulateRunSet => {
                    "Run Simulation preflight before dispatch; Studio runs are never auto-authorized"
                }
            };
            return Err(PreparationError::new(
                PreparationStage::Authorization,
                message,
            ));
        }

        let pending = self.pending.take().ok_or_else(|| {
            PreparationError::new(
                PreparationStage::Authorization,
                "No authorized prepared run is available",
            )
        })?;

        let current = match rebuild() {
            Ok(snapshot) => snapshot,
            Err(error) => {
                let _ = self.permits.invalidate();
                return Err(error);
            }
        };
        let retained_digest = pending.snapshot.digest();
        let current_digest = current.digest();
        let proof = match pending.permit.consume(retained_digest, current_digest) {
            Ok(proof) => proof,
            Err(error) => {
                let _ = self.permits.invalidate();
                return Err(PreparationError::new(
                    PreparationStage::Authorization,
                    format!(
                        "Prepared run expired because a bound input, capability, or check receipt changed ({error})"
                    ),
                ));
            }
        };
        pending.snapshot.authorize_dispatch(proof)
    }

    /// Campaign members were completely prepared together before the first
    /// dispatch. Authorize the frozen member without consulting live state.
    pub(in crate::simulation) fn authorize_campaign_member(
        &mut self,
        snapshot: PreparedRunSnapshot,
    ) -> Result<AuthorizedRunDispatch, String> {
        let digest = snapshot.digest();
        let permit = self
            .permits
            .issue(digest)
            .map_err(|error| format!("could not authorize member: {error}"))?;
        let proof = permit
            .consume(digest, digest)
            .map_err(|error| format!("could not consume member authorization: {error}"))?;
        snapshot
            .authorize_dispatch(proof)
            .map_err(|error| error.to_string())
    }
}
