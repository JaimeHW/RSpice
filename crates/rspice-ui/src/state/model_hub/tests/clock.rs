//! Unavailable catalog timing withholds offers without disabling local models.

use super::*;
use crate::time_compat::with_unix_epoch;
use std::time::Duration;

#[test]
fn fractional_catalog_validity_is_not_rounded_to_whole_seconds() {
    let key = hub_signing_key();
    let tree = TempTree::new("fractional-catalog-clock");
    let mut hub = filesystem_hub(&tree, &key);
    let snapshot = Snapshot {
        schema: rspice_pack::SNAPSHOT_SCHEMA,
        serial: 1,
        generated_at: "2026-08-15T09:30:00.500Z".to_owned(),
        expires_at: "2026-08-15T09:30:00.750Z".to_owned(),
        packs: Vec::new(),
        revocations: Vec::new(),
    };
    let signed = encode_snapshot(&snapshot, &key).unwrap();
    hub.refresh_catalog(&StubTransport::with_snapshot(signed))
        .unwrap();
    let second = 1_786_786_200_000;
    for fraction in [499, 500, 749, 750] {
        with_unix_epoch(
            Ok(Duration::from_millis(second + fraction)),
            || match fraction {
                499 => assert!(matches!(
                    hub.require_current_catalog(),
                    Err(ModelHubError::CatalogClockUnavailable(_))
                )),
                500 | 749 => {
                    hub.require_current_catalog().unwrap();
                    assert!(hub.offered_snapshot().is_some());
                    assert!(hub.catalog_expired().is_none());
                }
                _ => {
                    assert!(matches!(
                        hub.require_current_catalog(),
                        Err(ModelHubError::CatalogExpired { .. })
                    ));
                    assert!(hub.offered_snapshot().is_none());
                    assert_eq!(hub.catalog_expired(), Some(snapshot.expires_at.as_str()));
                }
            },
        );
    }
}

#[test]
fn invalid_catalog_clocks_preserve_installed_sources_and_local_execution() {
    let key = hub_signing_key();
    let archive = signed_archive(&key, &["subckt", "resistor"]);
    let tree = TempTree::new("invalid-catalog-clock");
    let mut hub = filesystem_hub(&tree, &key);
    let transport = StubTransport::with_snapshot(catalog_at(&key, &archive, 1))
        .serving(VERSION, archive.clone());
    hub.refresh_catalog(&transport).unwrap();
    hub.install(&transport, PACK_ID, VERSION).unwrap();
    let identity = hub.catalog_identity().unwrap().clone();
    let generated = identity.generated_at_epoch.unwrap();
    let cached = filesystem_store(&tree).read_snapshot().unwrap();

    for epoch in [
        Err("clock unavailable"),
        Ok(Duration::ZERO),
        Ok(Duration::MAX),
        Ok(generated - Duration::from_secs(1)),
    ] {
        with_unix_epoch(epoch, || {
            assert!(matches!(
                hub.require_current_catalog(),
                Err(ModelHubError::CatalogClockUnavailable(_))
            ));
            assert!(hub.offered_snapshot().is_none());
            assert!(matches!(
                hub.install(&transport, PACK_ID, VERSION),
                Err(ModelHubError::CatalogClockUnavailable(_))
            ));
            assert!(hub.snapshot().is_some());
            let parts = hub.part_index(&[]);
            assert!(
                parts
                    .iter()
                    .all(|row| !matches!(row.provenance, PartProvenance::RemoteRelease { .. }))
            );
            assert!(
                parts
                    .iter()
                    .any(|row| matches!(row.provenance, PartProvenance::InstalledPack { .. }))
            );
            hub.verify_installed(PACK_ID, VERSION).unwrap();
            let mut manager = crate::state::model_library::ModelLibraryManager::new();
            hub.add_part_to_project(&mut manager, PACK_ID, VERSION, PART_ID)
                .unwrap();
            assert!((retained_divider_output(&manager) - 0.5).abs() < 1.0e-9);
        });
        assert_eq!(hub.catalog_identity(), Some(&identity));
        assert_eq!(filesystem_store(&tree).read_snapshot().unwrap(), cached);
    }
    hub.require_current_catalog().unwrap();
    assert!(hub.offered_snapshot().is_some());

    // The pack format authenticates these spellings and checks their shape,
    // which is not a claim that either instant exists: the calendar has no
    // February 31, and Unix time cannot name a leap second. Neither becomes
    // an offer with no expiry, and neither is the pack format's fault.
    for (serial, expires_at) in [(2_u64, "2026-02-31T09:30:00Z"), (3, "2026-08-15T09:30:60Z")] {
        let unplaceable = signed_snapshot_on(
            &key,
            &[(VERSION, &archive, &["subckt", "resistor"])],
            &CatalogTerms {
                serial,
                expires_at: expires_at.to_owned(),
                ..CatalogTerms::default()
            },
        );
        hub.refresh_catalog(&StubTransport::with_snapshot(unplaceable))
            .unwrap();
        let Err(ModelHubError::CatalogInstantUnplaceable { field, instant }) =
            hub.require_current_catalog()
        else {
            panic!("{expires_at} must not authorize an offer")
        };
        assert_eq!(field, "expiry");
        assert_eq!(instant, expires_at);
        assert!(hub.offered_snapshot().is_none());
        hub.verify_installed(PACK_ID, VERSION).unwrap();
    }
}
