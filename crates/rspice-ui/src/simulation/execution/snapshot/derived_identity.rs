//! Where a dispatched task's identity comes from when the plan authored none.
//!
//! Preparation dispatches more tasks than the plan authored. A multi-point Run
//! Set and a declared corner or temperature space solve one point per task; a
//! PSS that retains harmonics earns a spectrum companion. Each of those tasks
//! needs its own identity — sharing the authored one would make every
//! dependent binding resolve the wrong task — and each must be able to say
//! where that identity came from, because a project may only persist a result
//! history that closes over the plan.
//!
//! So all of them are minted the same way: a [`DerivedAnalysisIdentity`]
//! rooted at the authored instance and extended by one role per derivation
//! step. The record travels on the task into its run receipt, and project load
//! re-derives the identity from it. Nothing else in preparation may give a
//! task an identity the plan did not author.

use crate::product::{AnalysisInstanceId, DerivedAnalysisIdentity, ObjectRevision};

use super::{PreparedPvtPoint, PreparedTask, QueuedAnalysis, corner_contract_digest, process_tag};

/// The role a PSS spectrum companion plays for the PSS it reads.
///
/// Role strings are part of persisted identity: a role may be added, but
/// renaming one renames every identity already derived through it.
pub(in crate::simulation) const PSS_SPECTRUM_ROLE: &str = "pss-spectrum";

/// The role one point of the Studio's global Run Set plays for the analysis
/// declared over it.
///
/// Every condition that makes the point a different solve is named, so two
/// points of one declaration can never collide and a re-preparation of the
/// same declaration lands on the same identities.
pub(in crate::simulation) fn global_run_set_point_role(
    index: usize,
    point: &PreparedPvtPoint,
) -> String {
    let mut role = format!(
        "rspice-global-run-set/v2/{index}/{}/{:016x}/{:016x}/{}",
        process_tag(point.process),
        point.voltage.map(f64::to_bits).unwrap_or_default(),
        point.temperature_celsius.to_bits(),
        point_corner_tag(point),
    );
    for (name, value) in point
        .parameter_overrides
        .iter()
        .chain(&point.source_overrides)
    {
        role.push('/');
        role.push_str(name);
        role.push('=');
        role.push_str(value);
    }
    role
}

/// The role one run point of an operating point bound to the run-set axis
/// plays for that operating point.
pub(in crate::simulation) fn operating_point_run_point_role(
    index: usize,
    count: usize,
    point: &PreparedPvtPoint,
) -> String {
    format!(
        "rspice-op-run-point/v2/{index}/{count}/{}/{:016x}/{:016x}/{}",
        process_tag(point.process),
        point.voltage.map(f64::to_bits).unwrap_or_default(),
        point.temperature_celsius.to_bits(),
        point_corner_tag(point),
    )
}

fn point_corner_tag(point: &PreparedPvtPoint) -> String {
    point
        .corner_contract
        .as_ref()
        .map(corner_contract_digest)
        .map_or_else(|| "none".to_owned(), |digest| digest.to_string())
}

impl PreparedTask {
    /// A companion task derived from an authored one in a single step.
    pub(in crate::simulation) fn derived(
        producer: AnalysisInstanceId,
        role: &str,
        source_revision: ObjectRevision,
        dependencies: Vec<AnalysisInstanceId>,
        label: impl Into<String>,
        task: QueuedAnalysis,
    ) -> Self {
        let derivation = DerivedAnalysisIdentity::new(producer, role)
            .expect("a derivation role is a non-empty literal");
        let mut prepared = Self::new(
            derivation.instance_id(),
            source_revision,
            dependencies,
            label,
            task,
        );
        prepared.derivation = Some(derivation);
        prepared
    }

    /// One further derivation step away from this task.
    ///
    /// A task that is itself derived extends its own path rather than starting
    /// a new one, so the spectrum of a PSS solved at one point of a Run Set
    /// still roots at the PSS the plan authored.
    pub(in crate::simulation) fn derive(&self, role: impl Into<String>) -> DerivedAnalysisIdentity {
        match self.derivation.clone() {
            Some(path) => path.extended(role),
            None => DerivedAnalysisIdentity::new(self.instance_id, role),
        }
        .expect("a derivation role is a non-empty literal")
    }

    /// Take one derivation step, adopting the identity it names.
    ///
    /// Identity and record move together because they must: a task holding one
    /// without the other is exactly the state the receipt layer refuses.
    pub(in crate::simulation) fn adopt_derived_identity(
        &mut self,
        derivation: DerivedAnalysisIdentity,
    ) {
        self.instance_id = derivation.instance_id();
        self.derivation = Some(derivation);
    }
}
