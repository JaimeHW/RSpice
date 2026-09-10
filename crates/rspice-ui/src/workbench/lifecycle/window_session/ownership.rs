//! JSON-safe document ownership, retaining compatibility with old RON maps.

use super::{ApplicationWindowId, HashMap, WorkspaceDocumentId, document_sort_key};
use serde::{Serialize, de};

type Ownership = HashMap<WorkspaceDocumentId, ApplicationWindowId>;

pub(super) fn serialize<S: serde::Serializer>(
    ownership: &Ownership,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    // Document IDs are enums with payloads, so they cannot be JSON map keys.
    let mut entries: Vec<_> = ownership.iter().collect();
    entries.sort_by_key(|(document, _)| document_sort_key(document));
    entries.serialize(serializer)
}

pub(super) fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Ownership, D::Error> {
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = Ownership;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("document/window pairs or a legacy ownership map")
        }

        fn visit_seq<A: de::SeqAccess<'de>>(self, mut entries: A) -> Result<Ownership, A::Error> {
            let mut ownership = Ownership::new();
            while let Some((document, window)) = entries.next_element()? {
                insert(&mut ownership, document, window)?;
            }
            Ok(ownership)
        }

        fn visit_map<A: de::MapAccess<'de>>(self, mut entries: A) -> Result<Ownership, A::Error> {
            let mut ownership = Ownership::new();
            while let Some((document, window)) = entries.next_entry()? {
                insert(&mut ownership, document, window)?;
            }
            Ok(ownership)
        }
    }
    deserializer.deserialize_any(Visitor)
}

fn insert<E: de::Error>(
    ownership: &mut Ownership,
    document: WorkspaceDocumentId,
    window: ApplicationWindowId,
) -> Result<(), E> {
    if ownership.insert(document, window).is_some() {
        return Err(E::custom("duplicate document ownership"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::CellViewRef;
    use crate::workbench::lifecycle::window_session::WindowSessionRegistry;
    use crate::workbench::state::WorkspaceLayoutState;
    use serde::Deserialize;

    #[test]
    fn ownership_survives_json_ron_and_legacy_map_sessions() {
        let mut session = WindowSessionRegistry::default();
        let document = WorkspaceDocumentId::CellView(CellViewRef::new("work", "amp", "schematic"));
        let window = session
            .detach_document(
                document.clone(),
                "Amplifier",
                WorkspaceLayoutState::default(),
                true,
            )
            .unwrap();
        let json = serde_json::to_string(&session).unwrap();
        let ron = ron::to_string(&session).unwrap();
        for mut restored in [
            serde_json::from_str::<WindowSessionRegistry>(&json).unwrap(),
            ron::from_str::<WindowSessionRegistry>(&ron).unwrap(),
        ] {
            restored.normalize_after_restore();
            assert_eq!(restored.owner(&document), window);
            assert_eq!(
                restored.state(window).unwrap().active_document.as_ref(),
                Some(&document)
            );
        }
        // Emit the former field shape with a typed map so the test exercises
        // actual enum payloads in old eframe sessions.
        #[derive(Serialize)]
        struct Legacy<'a> {
            ownership: &'a Ownership,
        }
        let legacy = ron::to_string(&Legacy {
            ownership: &session.ownership,
        })
        .unwrap();
        let restored: WindowSessionRegistry = ron::from_str(&legacy).unwrap();
        assert_eq!(restored.ownership, session.ownership);
        let empty_legacy: WindowSessionRegistry =
            serde_json::from_str(r#"{"ownership":{}}"#).unwrap();
        assert!(empty_legacy.ownership.is_empty());
    }

    #[test]
    fn ownership_rejects_duplicate_documents() {
        #[derive(Deserialize)]
        struct Envelope {
            #[serde(with = "super")]
            _ownership: Ownership,
        }
        let pair = serde_json::json!([
            WorkspaceDocumentId::CellView(CellViewRef::new("work", "amp", "schematic")),
            2,
        ]);
        let value = serde_json::json!({"_ownership": [pair, pair]});
        let error = serde_json::from_value::<Envelope>(value).err().unwrap();
        assert!(error.to_string().contains("duplicate document ownership"));
    }
}
