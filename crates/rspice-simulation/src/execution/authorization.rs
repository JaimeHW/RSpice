//! Retained preflight authorization and single-use dispatch.

use super::permit::{ExecutionPermit, ExecutionPermitIssuer};
use super::{AuthorizedRunDispatch, PreparationError, PreparationStage, PreparedRunSnapshot};
use crate::execution::SimulationRunIntent;

struct PendingPreparedRun {
    snapshot: PreparedRunSnapshot,
    permit: ExecutionPermit,
}

/// Owns both the retained snapshot and the authority that can consume it.
/// Preparation stays with the owning workflow, which retains what the
/// snapshot was prepared from; before releasing a dispatch this boundary has
/// the owner confirm that none of it has moved.
#[derive(Default)]
pub struct PreparedRunAuthorization {
    pending: Option<PendingPreparedRun>,
    permits: ExecutionPermitIssuer,
}

impl PreparedRunAuthorization {
    pub fn clear(&mut self) {
        self.pending = None;
        if let Err(error) = self.permits.invalidate() {
            log::error!("Failed to invalidate prepared execution permit: {error}");
        }
    }

    pub fn retained_snapshot(&self) -> Option<&PreparedRunSnapshot> {
        self.pending.as_ref().map(|pending| &pending.snapshot)
    }

    pub fn retain(&mut self, snapshot: PreparedRunSnapshot) -> Result<(), PreparationError> {
        let permit = self.permits.issue(snapshot.digest()).map_err(|error| {
            PreparationError::new(
                PreparationStage::Authorization,
                format!("Could not authorize prepared run: {error}"),
            )
        })?;
        self.pending = Some(PendingPreparedRun { snapshot, permit });
        Ok(())
    }

    /// Dispatch the retained snapshot itself, without preparing it again.
    ///
    /// The snapshot's own preparation is the authorization: it was prepared
    /// from the session as it stood at one instant and checked, and the owner
    /// retained the identity of those inputs beside it. `verify` confirms
    /// that nothing the snapshot was prepared from has moved since. When it
    /// refuses, the retained snapshot and its permit are spent and nothing is
    /// dispatched.
    pub fn consume(
        &mut self,
        intent: SimulationRunIntent,
        verify: impl FnOnce(&PreparedRunSnapshot) -> Result<(), PreparationError>,
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
        if let Err(error) = verify(&pending.snapshot) {
            let _ = self.permits.invalidate();
            return Err(error);
        }
        let digest = pending.snapshot.digest();
        let proof = match pending.permit.consume(digest, digest) {
            Ok(proof) => proof,
            Err(error) => {
                let _ = self.permits.invalidate();
                return Err(PreparationError::new(
                    PreparationStage::Authorization,
                    format!("Prepared run could not be dispatched ({error})"),
                ));
            }
        };
        pending.snapshot.authorize_dispatch(proof)
    }

    /// Campaign members were completely prepared together before the first
    /// dispatch. Authorize the frozen member without consulting live state.
    pub fn authorize_campaign_member(
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
