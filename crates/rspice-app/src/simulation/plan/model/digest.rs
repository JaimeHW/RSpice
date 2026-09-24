//! The digest a configuration receipt carries.
//!
//! A receipt already names the revision it moved the plan from and to. A
//! revision is a counter, though: two projects that diverged both have a
//! revision 7, and neither receipt can tell you whether they adopted the same
//! plan. The digest answers that — it is taken over the plan as adopted, so two
//! receipts agree exactly when the plans they adopted agree.

use std::fmt::Write as _;

use super::SimulationPlan;

const OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
const PRIME: u64 = 0x0000_0100_0000_01b3;

/// FNV-1a/64 over the plan this receipt adopted, in the display form the
/// run-set receipts already use.
///
/// What the record covers, and why that is the whole plan:
///
/// * The plan identity and its committed revision, so a digest cannot be read
///   as belonging to a different plan or a different point in its history.
/// * Every instance in presentation order: position, identity, canonical kind
///   tag, name, enabled flag, dependency edges, and whether it departs from the
///   plan's solver policy. These are the facts a person can change from the
///   registry pages.
/// * Each instance's modified revision. This is what makes the record sensitive
///   to *draft* values without serializing them: a draft edit is a plan command
///   and every plan command stamps the instance it touched. A draft that
///   changed and a draft that did not are therefore never the same record.
///
/// The tag comes from [`CanonicalAnalysisKind`], never a spelled-out literal,
/// so a kind cannot be renamed in prose and silently keep its digest.
///
/// [`CanonicalAnalysisKind`]: crate::state::simulation::CanonicalAnalysisKind
pub(super) fn adopted_plan_digest(plan: &SimulationPlan) -> String {
    let mut record = format!("plan|{}|{}", plan.id, plan.revision.get());
    for (position, instance) in plan.instances.iter().enumerate() {
        let _ = write!(
            record,
            "\u{1e}{position}|{}|{}|{}|{}|{}|{}",
            instance.id,
            instance.kind.canonical_kind().tag(),
            instance.name.as_deref().unwrap_or(""),
            u8::from(instance.enabled),
            instance.modified_revision.get(),
            u8::from(instance.numeric_override.is_some()),
        );
        for dependency in &instance.dependencies {
            let _ = write!(
                record,
                "\u{1f}{}|{}",
                dependency.prerequisite().canonical_kind().tag(),
                dependency.target()
            );
        }
    }
    format!("fnv1a64-{:016x}", fnv1a64(record.as_bytes()))
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = OFFSET_BASIS;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::plan::{AnalysisDraft, AnalysisKind};

    fn transient_plan() -> SimulationPlan {
        let mut plan = SimulationPlan::empty();
        plan.insert(AnalysisKind::Transient).expect("inserts");
        plan
    }

    fn commit(plan: &mut SimulationPlan, detail: &str) -> String {
        plan.commit_configuration_change(detail)
            .expect("the detail is a single non-empty line")
            .digest()
            .expect("a receipt written now carries a digest")
            .to_owned()
    }

    #[test]
    fn a_digest_is_the_run_set_display_form() {
        let digest = commit(&mut transient_plan(), "Added design variable vdd.");
        assert!(
            digest.starts_with("fnv1a64-") && digest.len() == "fnv1a64-".len() + 16,
            "{digest} is not the shared display form"
        );
        assert!(
            digest["fnv1a64-".len()..]
                .chars()
                .all(|c| c.is_ascii_hexdigit())
        );
    }

    #[test]
    fn two_plans_that_agree_produce_the_same_digest() {
        let mut first = transient_plan();
        let mut second = first.clone();
        assert_eq!(
            commit(&mut first, "Added design variable vdd."),
            commit(&mut second, "Added design variable vdd."),
        );
    }

    #[test]
    fn the_digest_follows_a_rename() {
        let mut plan = transient_plan();
        let before = commit(&mut plan, "Added design variable vdd.");
        let id = plan.instances()[0].id();
        plan.set_instance_name(id, "Startup").expect("free name");
        let after = commit(&mut plan, "Added design variable vdd.");
        assert_ne!(before, after, "a renamed instance is a different plan");
    }

    #[test]
    fn the_digest_follows_a_draft_edit_through_the_instance_revision() {
        let mut plan = transient_plan();
        let before = commit(&mut plan, "Added design variable vdd.");
        let id = plan.instances()[0].id();
        plan.edit(id, |draft| {
            let AnalysisDraft::Transient(draft) = draft else {
                panic!("expected a Transient draft");
            };
            draft.stop = "2u".to_owned();
        })
        .expect("a draft edit commits");
        let after = commit(&mut plan, "Added design variable vdd.");
        assert_ne!(before, after, "an edited draft is a different plan");
    }

    #[test]
    fn the_digest_follows_the_enabled_set() {
        let mut plan = transient_plan();
        plan.insert(AnalysisKind::Ac).expect("inserts");
        let before = commit(&mut plan, "Added design variable vdd.");
        let ac = plan.instances()[1].id();
        plan.set_enabled(ac, false).expect("disables");
        let after = commit(&mut plan, "Added design variable vdd.");
        assert_ne!(before, after, "a disabled analysis is a different plan");
    }

    /// The same case with the key genuinely gone, not rewritten to `""`.
    ///
    /// A receipt written before digests existed carries no `digest` member at
    /// all. Rewriting the value to an empty string exercises the parser's
    /// handling of a *present* field, which is a different question: it would
    /// pass even if the field had no `serde(default)` and every such project
    /// failed to open.
    #[test]
    fn a_receipt_with_no_digest_key_at_all_still_loads() {
        let mut plan = transient_plan();
        plan.commit_configuration_change("Added design variable vdd.")
            .expect("commits");
        let mut document: serde_json::Value =
            serde_json::to_value(&plan).expect("a plan serializes to JSON");
        let receipt = document["configuration_receipts"][0]
            .as_object_mut()
            .expect("receipts are written as objects");
        assert!(
            receipt.remove("digest").is_some(),
            "a fresh receipt states a digest, so removing it is a real regression to an \
             older document"
        );

        let restored: SimulationPlan =
            serde_json::from_value(document).expect("an older receipt still loads");
        assert_eq!(restored.configuration_receipts()[0].digest(), None);
        assert!(
            !restored.configuration_receipts()[0]
                .status_line()
                .contains("fnv1a64"),
            "a receipt without a digest does not claim one"
        );
    }

    #[test]
    fn a_receipt_persisted_before_digests_reports_absence_not_a_wrong_digest() {
        let mut plan = transient_plan();
        plan.commit_configuration_change("Added design variable vdd.")
            .expect("commits");
        let json = serde_json::to_string(&plan).expect("a plan serializes");
        let stripped = json.replace(
            &format!(
                "\"digest\":\"{}\"",
                plan.configuration_receipts()[0]
                    .digest()
                    .expect("a fresh receipt has one")
            ),
            "\"digest\":\"\"",
        );
        let restored: SimulationPlan =
            serde_json::from_str(&stripped).expect("an older receipt still loads");
        assert_eq!(restored.configuration_receipts()[0].digest(), None);
        assert!(
            !restored.configuration_receipts()[0]
                .status_line()
                .contains("fnv1a64"),
            "a receipt without a digest does not claim one"
        );
    }
}
