//! Cooperative Active-region execution over the existing HDL scheduler.
//! The owning circuit must retain the participant and roll it back with the host;
//! this interface neither launches a second simulator nor promotes an HDL region.
use super::super::store::{DigitalBitChange, ExternalBitDriverId};
use super::*;

/// One external Active wave. Return true while external work remains, even if
/// it did not change an HDL bit. Consume resolved changes in publication order;
/// reading only the last value can lose intervening edges.
pub(crate) trait DigitalActiveParticipant {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError>;
}

/// Restricted access: an event participant can observe resolved nets and drive
/// its enrolled outputs, but cannot advance HDL time or promote a later region.
pub(crate) struct DigitalActiveExchange<'a> {
    pub(super) host: &'a mut DigitalHost,
    pub(super) tick: u64,
    pub(super) physical_seconds: f64,
}

impl DigitalActiveExchange<'_> {
    pub(crate) fn tick(&self) -> u64 {
        self.tick
    }
    pub(crate) fn physical_seconds(&self) -> f64 {
        self.physical_seconds
    }
    pub(crate) fn read_net(&self, net: usize) -> Result<DigitalValue, DigitalRunError> {
        self.host
            .store
            .connected_value(net)
            .ok_or_else(|| DigitalRunError::ExternalExecution {
                detail: format!("unknown shared digital net {net}"),
            })
    }
    pub(crate) fn take_changes(&mut self) -> Vec<DigitalBitChange> {
        self.host.store.take_external_bit_changes()
    }
    pub(crate) fn drive_many(
        &mut self,
        drives: &[(ExternalBitDriverId, DigitalValue)],
    ) -> Result<(), DigitalRunError> {
        self.host
            .store
            .check_external_drives(drives)
            .map_err(|detail| DigitalRunError::ExternalExecution { detail })?;
        for (driver, _) in drives {
            self.host
                .scheduler
                .note_external_activation(self.tick, self.host.external_targets[driver.index()])?;
        }
        self.host.store.publish_external_drives(drives);
        Ok(())
    }
}

pub(super) struct NoActiveParticipant;
impl DigitalActiveParticipant for NoActiveParticipant {
    fn settle_active(
        &mut self,
        exchange: &mut DigitalActiveExchange<'_>,
    ) -> Result<bool, DigitalRunError> {
        exchange.host.require_standalone_execution()?;
        Ok(false)
    }
}

impl DigitalHost {
    pub(super) fn require_standalone_execution(&self) -> Result<(), DigitalRunError> {
        if self.store.has_external_participants() {
            return Err(DigitalRunError::ExternalExecution {
                detail: "enrolled event drivers require their circuit Active participant".into(),
            });
        }
        Ok(())
    }

    pub(crate) fn is_external_target(&self, target: &EventTarget) -> bool {
        self.store
            .external_sources()
            .iter()
            .any(|(_, external)| external == target)
    }

    /// Enroll external driver identities at elaboration, before process execution.
    /// The returned IDs and immutable topology survive fresh analysis and cloning.
    pub(crate) fn attach_external_bits(
        &mut self,
        observed: &[usize],
        drivers: &[(usize, EventTarget)],
    ) -> Result<Vec<ExternalBitDriverId>, DigitalRunError> {
        if self.elaboration_closed {
            return Err(DigitalRunError::ExternalExecution {
                detail: "event topology cannot change after digital execution starts".into(),
            });
        }
        let identities = self
            .store
            .attach_external_bits(observed, drivers)
            .map_err(|detail| DigitalRunError::ExternalExecution { detail })?;
        self.external_targets = drivers
            .iter()
            .map(|(_, target)| self.scheduler.intern_target(target.clone()))
            .collect();
        Ok(identities)
    }
}
