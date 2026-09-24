//! Reading a device-local session that names a document this build retired.
//!
//! [`WorkspaceDocumentId`] is persisted — the open-document registry, the
//! window session's ownership map, and each window's own document list all
//! carry it — and `ron` refuses an unknown enum variant outright. So deleting
//! a variant is not free: `AppState::restore_eframe_session` falls back to
//! `default()` on any error, which would cost a reader upgrading across the
//! deletion their whole session rather than one stale tab.
//!
//! [`RetirableDocumentId`] is the tolerant read. It takes the variant name
//! first: a retired one is consumed and reported as `None` for its holder to
//! drop, and every other name is handed straight back to the derived
//! implementation, so this module never has to know what the live variants
//! are and cannot drift from them.

use std::collections::{HashMap, HashSet};
use std::fmt;

use serde::Deserialize;
use serde::de::{
    self, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, VariantAccess, Visitor,
};

use super::{Workspace, WorkspaceDocumentId};

/// The document identities the product no longer presents, spelled the way an
/// older session wrote them.
///
/// 2026-09-16 `AnalysisSetup`: the Simulation Studio's per-analysis
/// "<name> · setup" tab. The Studio is one document now, so a session that
/// names one opens on the plan instead.
const RETIRED_VARIANTS: [&str; 1] = ["AnalysisSetup"];

/// One persisted document identity, or `None` when the session named a
/// retired one.
pub(crate) struct RetirableDocumentId(pub(crate) Option<WorkspaceDocumentId>);

impl<'de> Deserialize<'de> for RetirableDocumentId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // The variant list is the derived implementation's business; this one
        // dispatches on the name it reads, so it passes none of its own.
        deserializer.deserialize_enum("WorkspaceDocumentId", &[], RetirableVisitor)
    }
}

struct RetirableVisitor;

impl<'de> Visitor<'de> for RetirableVisitor {
    type Value = RetirableDocumentId;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a workspace document identity")
    }

    fn visit_enum<A: EnumAccess<'de>>(self, data: A) -> Result<Self::Value, A::Error> {
        let (VariantName(name), variant) = data.variant::<VariantName>()?;
        if RETIRED_VARIANTS.contains(&name.as_str()) {
            // Every retired identity was a newtype variant over an object ID.
            // The payload is still read rather than left in the stream: the
            // fields after it belong to the caller.
            variant.newtype_variant::<de::IgnoredAny>()?;
            return Ok(RetirableDocumentId(None));
        }
        WorkspaceDocumentId::deserialize(ReplayVariant { name, variant })
            .map(|document| RetirableDocumentId(Some(document)))
    }
}

/// An enum's variant name, read the way derived code reads one.
///
/// Not `String`: `String::deserialize` asks for `deserialize_string`, and a
/// variant name is not a string in every format that can hold one — RON
/// answers `deserialize_identifier` and refuses a bare identifier asked for as
/// a string (`ExpectedString`). Serde's own generated field readers call
/// `deserialize_identifier`, and JSON forwards that to its map key, so the one
/// call covers both wire formats the session is written in.
struct VariantName(String);

impl<'de> Deserialize<'de> for VariantName {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_identifier(VariantNameVisitor)
    }
}

struct VariantNameVisitor;

impl Visitor<'_> for VariantNameVisitor {
    type Value = VariantName;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a variant name")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<VariantName, E> {
        Ok(VariantName(value.to_owned()))
    }

    fn visit_bytes<E: de::Error>(self, value: &[u8]) -> Result<VariantName, E> {
        std::str::from_utf8(value)
            .map(|name| VariantName(name.to_owned()))
            .map_err(|_| de::Error::invalid_value(de::Unexpected::Bytes(value), &self))
    }
}

/// The variant this visitor already consumed, offered to the derived
/// implementation as if it had never been read.
struct ReplayVariant<A> {
    name: String,
    variant: A,
}

impl<'de, A: VariantAccess<'de>> Deserializer<'de> for ReplayVariant<A> {
    type Error = A::Error;

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error> {
        visitor.visit_enum(self)
    }

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value, Self::Error> {
        Err(de::Error::custom(
            "a workspace document identity is read as an enum",
        ))
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map struct identifier ignored_any
    }
}

impl<'de, A: VariantAccess<'de>> EnumAccess<'de> for ReplayVariant<A> {
    type Error = A::Error;
    type Variant = A;

    fn variant_seed<S: DeserializeSeed<'de>>(self, seed: S) -> Result<(S::Value, A), Self::Error> {
        let name: de::value::StringDeserializer<A::Error> = self.name.into_deserializer();
        Ok((seed.deserialize(name)?, self.variant))
    }
}

/// An `Option<WorkspaceDocumentId>` field: a retired identity reads as no
/// document at all, which is what its holder would have shown anyway.
pub(crate) fn deserialize_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<WorkspaceDocumentId>, D::Error> {
    Ok(Option::<RetirableDocumentId>::deserialize(deserializer)?.and_then(|document| document.0))
}

/// A `HashSet<WorkspaceDocumentId>` field, minus the retired identities.
pub(crate) fn deserialize_set<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<HashSet<WorkspaceDocumentId>, D::Error> {
    Ok(Vec::<RetirableDocumentId>::deserialize(deserializer)?
        .into_iter()
        .filter_map(|document| document.0)
        .collect())
}

/// A `Vec<WorkspaceDocumentId>` field, minus the retired identities.
pub(crate) fn deserialize_list<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<WorkspaceDocumentId>, D::Error> {
    Ok(Vec::<RetirableDocumentId>::deserialize(deserializer)?
        .into_iter()
        .filter_map(|document| document.0)
        .collect())
}

/// The active document of each workspace, minus the workspaces whose active
/// document was a retired identity. Dropping the entry is what leaves the
/// Studio on its plan: `authoritative_active_document` falls back to the
/// first available document when the registry names none.
pub(crate) fn deserialize_active_map<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<HashMap<Workspace, WorkspaceDocumentId>, D::Error> {
    Ok(
        HashMap::<Workspace, RetirableDocumentId>::deserialize(deserializer)?
            .into_iter()
            .filter_map(|(workspace, document)| Some((workspace, document.0?)))
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::CellViewRef;

    /// The retired Studio setup tab reads as no document, and its neighbours
    /// in the same session survive it.
    ///
    /// Both wire formats, because the window session is written as RON by
    /// eframe and as JSON by the ownership field's own compatibility path.
    #[test]
    fn a_retired_identity_reads_as_no_document_beside_live_ones() {
        const RON: &str = r#"[
            AnalysisSetup("9f1b6f6c-0f1a-4f2e-9c6d-2a1b3c4d5e6f"),
            SimulationPlan,
            CellView((library: "work", cell: "amp", view: "schematic")),
        ]"#;
        let documents =
            deserialize_list(&mut ron::Deserializer::from_str(RON).expect("the fixture is RON"))
                .expect("a retired identity is tolerated, not refused");
        assert_eq!(
            documents,
            vec![
                WorkspaceDocumentId::SimulationPlan,
                WorkspaceDocumentId::CellView(CellViewRef::new("work", "amp", "schematic")),
            ]
        );

        const JSON: &str = r#"[
            {"AnalysisSetup": "9f1b6f6c-0f1a-4f2e-9c6d-2a1b3c4d5e6f"},
            "SimulationPlan"
        ]"#;
        let documents = deserialize_list(&mut serde_json::Deserializer::from_str(JSON))
            .expect("a retired identity is tolerated in JSON too");
        assert_eq!(documents, vec![WorkspaceDocumentId::SimulationPlan]);
    }

    /// A variant nobody has ever written is still an error: tolerance is a
    /// list of identities this build retired, not a licence to read anything.
    #[test]
    fn an_unknown_identity_is_still_refused() {
        let error = deserialize_option(
            &mut ron::Deserializer::from_str("Some(SomethingElse(1))").expect("the fixture is RON"),
        )
        .expect_err("an unknown variant has no meaning to recover");
        assert!(
            error.to_string().contains("SomethingElse"),
            "the refusal names what it could not read: {error}"
        );
    }
}
