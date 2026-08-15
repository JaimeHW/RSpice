use std::collections::{BTreeMap, BTreeSet};

use crate::{
    error::PackError,
    limits::Limits,
    manifest::{
        FileEntry, MANIFEST_SCHEMA, Manifest, ManifestTemplate, RESERVED_ENTRIES,
        canonical_manifest_bytes, parse_manifest,
    },
    path::{MANIFEST_ENTRY, SIGNATURE_ENTRY, is_reserved_path, validate_path},
    signing::{
        SIGNATURE_BYTES, SigningKey, VerifyingKey, sha256_hex, sign_manifest,
        verify_manifest_signature,
    },
    zip::{read_archive, write_archive},
};

/// A pack whose signature, canonical manifest, and every listed file have been
/// proved against one verifying key.
///
/// Holding this value is the evidence: there is no way to construct it without
/// passing the whole pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedPack {
    pub manifest: Manifest,
    /// Expanded contents of every file the manifest lists, keyed by path. The
    /// two container-owned entries are not included.
    pub files: BTreeMap<String, Vec<u8>>,
}

/// Whole-archive operations over the `.rspicepack` container.
pub struct Pack;

impl Pack {
    /// Verifies an untrusted archive end to end.
    ///
    /// The order is the security argument: nothing about the manifest is
    /// believed until its signature is proved over the exact stored bytes, and
    /// no file content is believed until its digest is proved against the
    /// signed manifest. The two file-set directions are both checked, so a
    /// pack can neither hide an unlisted payload nor promise a file it omits.
    pub fn verify(
        archive: &[u8],
        key: &VerifyingKey,
        limits: &Limits,
    ) -> Result<VerifiedPack, PackError> {
        let mut manifest_bytes = None;
        let mut signature_bytes = None;
        let mut files = BTreeMap::new();
        for entry in read_archive(archive, limits)? {
            match entry.name.as_str() {
                MANIFEST_ENTRY => manifest_bytes = Some(entry.data),
                SIGNATURE_ENTRY => signature_bytes = Some(entry.data),
                _ => {
                    files.insert(entry.name, entry.data);
                }
            }
        }
        let manifest_bytes =
            manifest_bytes.ok_or(PackError::MissingReservedEntry(MANIFEST_ENTRY))?;
        let signature_bytes =
            signature_bytes.ok_or(PackError::MissingReservedEntry(SIGNATURE_ENTRY))?;
        let signature: [u8; SIGNATURE_BYTES] =
            signature_bytes
                .as_slice()
                .try_into()
                .map_err(|_| PackError::MalformedSignature {
                    expected: SIGNATURE_BYTES,
                    actual: signature_bytes.len(),
                })?;
        verify_manifest_signature(&manifest_bytes, &signature, key)?;
        let manifest = parse_manifest(&manifest_bytes)?;

        for entry in &manifest.files {
            let Some(content) = files.get(&entry.path) else {
                return Err(PackError::MissingListedFile(entry.path.clone()));
            };
            if content.len() as u64 != entry.length {
                return Err(PackError::LengthMismatch {
                    path: entry.path.clone(),
                    expected: entry.length,
                    actual: content.len() as u64,
                });
            }
            if sha256_hex(content) != entry.sha256 {
                return Err(PackError::DigestMismatch {
                    path: entry.path.clone(),
                });
            }
        }
        let listed: BTreeSet<&str> = manifest
            .files
            .iter()
            .map(|entry| entry.path.as_str())
            .collect();
        for name in files.keys() {
            if !listed.contains(name.as_str()) {
                return Err(PackError::UnlistedEntry(name.clone()));
            }
        }
        Ok(VerifiedPack { manifest, files })
    }
}

/// Builds and signs a pack from an in-memory file set.
///
/// The spec limits are enforced here rather than left to the verifier, so the
/// packer cannot emit an archive that no client would accept.
pub fn build_pack(
    files: &[(String, Vec<u8>)],
    template: ManifestTemplate,
    key: &SigningKey,
) -> Result<Vec<u8>, PackError> {
    let limits = Limits::default();
    if files.len() + RESERVED_ENTRIES.len() > limits.max_files {
        return Err(PackError::TooManyFiles {
            limit: limits.max_files,
            actual: files.len() + RESERVED_ENTRIES.len(),
        });
    }
    let mut sorted: Vec<(&str, &[u8])> = Vec::with_capacity(files.len());
    let mut expanded = 0_u64;
    for (path, content) in files {
        validate_path(path)?;
        if is_reserved_path(path) {
            return Err(PackError::ReservedFileName(path.clone()));
        }
        expanded += content.len() as u64;
        sorted.push((path.as_str(), content.as_slice()));
    }
    if expanded > limits.max_expanded_bytes as u64 {
        return Err(PackError::ExpandedTooLarge {
            limit: limits.max_expanded_bytes,
        });
    }
    sorted.sort_unstable_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    for pair in sorted.windows(2) {
        if let [left, right] = pair
            && left.0 == right.0
        {
            return Err(PackError::DuplicateEntry(left.0.to_owned()));
        }
    }

    let manifest = Manifest {
        schema: MANIFEST_SCHEMA,
        pack: template.pack,
        license: template.license,
        requires: template.requires,
        files: sorted
            .iter()
            .map(|(path, content)| FileEntry {
                path: (*path).to_owned(),
                length: content.len() as u64,
                sha256: sha256_hex(content),
            })
            .collect(),
        parts: template.parts,
    };
    let manifest_bytes = canonical_manifest_bytes(&manifest)?;
    let signature = sign_manifest(&manifest_bytes, key);

    let mut entries: Vec<(&str, &[u8])> = Vec::with_capacity(sorted.len() + 2);
    entries.push((MANIFEST_ENTRY, manifest_bytes.as_slice()));
    entries.push((SIGNATURE_ENTRY, signature.as_slice()));
    entries.extend_from_slice(&sorted);
    let archive = write_archive(&entries)?;
    if archive.len() > limits.max_archive_bytes {
        return Err(PackError::ArchiveTooLarge {
            limit: limits.max_archive_bytes,
            actual: archive.len(),
        });
    }
    Ok(archive)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        signing::signing_key,
        testing,
        zip::tests::{
            CENTRAL_FLAGS, CENTRAL_METHOD, build_stored_archive, central_offset, patch_u16,
        },
    };

    fn pack() -> Vec<u8> {
        build_pack(
            &testing::files(),
            testing::template(),
            &signing_key(&testing::SECRET),
        )
        .expect("the reference pack builds")
    }

    /// Expands an archive into an owned, mutable entry list.
    fn entries_of(archive: &[u8]) -> Vec<(String, Vec<u8>)> {
        read_archive(archive, &Limits::default())
            .expect("the reference archive reads")
            .into_iter()
            .map(|entry| (entry.name, entry.data))
            .collect()
    }

    /// Rewrites an entry list back into the container's fixed order.
    fn assemble(mut entries: Vec<(String, Vec<u8>)>) -> Vec<u8> {
        entries.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
        entries.sort_by_key(|(name, _)| match name.as_str() {
            MANIFEST_ENTRY => 0,
            SIGNATURE_ENTRY => 1,
            _ => 2,
        });
        let borrowed: Vec<(&str, &[u8])> = entries
            .iter()
            .map(|(name, data)| (name.as_str(), data.as_slice()))
            .collect();
        write_archive(&borrowed).expect("the rewritten archive writes")
    }

    fn content_of<'a>(entries: &'a [(String, Vec<u8>)], name: &str) -> &'a [u8] {
        entries
            .iter()
            .find(|(entry, _)| entry == name)
            .map(|(_, data)| data.as_slice())
            .expect("the reference archive carries the entry")
    }

    #[test]
    fn a_built_pack_verifies_and_is_byte_reproducible() {
        let key = signing_key(&testing::SECRET);
        let files = testing::files();
        assert_eq!(pack(), pack());

        let archive = pack();
        let verified =
            Pack::verify(&archive, &key.verifying_key(), &Limits::default()).expect("verifies");
        assert_eq!(verified.files.len(), files.len());
        for (path, content) in &files {
            assert_eq!(verified.files.get(path), Some(content));
        }
        assert_eq!(verified.manifest.pack.id, "rspice-opamps");
        assert_eq!(verified.manifest.files.len(), files.len());
    }

    #[test]
    fn the_container_entries_lead_the_archive_in_a_fixed_order() {
        let archive = pack();
        let entries = entries_of(&archive);
        let names: Vec<&str> = entries.iter().map(|(name, _)| name.as_str()).collect();
        assert_eq!(names[0], MANIFEST_ENTRY);
        assert_eq!(names[1], SIGNATURE_ENTRY);
        let mut rest = names[2..].to_vec();
        rest.sort_unstable();
        assert_eq!(&names[2..], rest.as_slice());
    }

    #[test]
    fn a_flipped_content_byte_fails_its_digest() {
        let key = signing_key(&testing::SECRET);
        let mut entries = entries_of(&pack());
        for (name, data) in &mut entries {
            if name == "models/opamps.lib" {
                data[0] ^= 0x20;
            }
        }
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::DigestMismatch { path }) if path == "models/opamps.lib"
        ));
    }

    #[test]
    fn a_changed_length_is_reported_before_the_digest() {
        let key = signing_key(&testing::SECRET);
        let mut entries = entries_of(&pack());
        for (name, data) in &mut entries {
            if name == "models/opamps.lib" {
                data.push(b'\n');
            }
        }
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::LengthMismatch { path, .. }) if path == "models/opamps.lib"
        ));
    }

    #[test]
    fn a_flipped_signature_byte_and_a_foreign_key_are_both_refused() {
        let key = signing_key(&testing::SECRET);
        let foreign = signing_key(&[0x5a_u8; 32]);
        let archive = pack();
        assert!(matches!(
            Pack::verify(&archive, &foreign.verifying_key(), &Limits::default()),
            Err(PackError::BadSignature)
        ));

        let mut entries = entries_of(&archive);
        for (name, data) in &mut entries {
            if name == SIGNATURE_ENTRY {
                data[10] ^= 0x01;
            }
        }
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::BadSignature)
        ));
    }

    #[test]
    fn a_truncated_signature_entry_is_refused() {
        let key = signing_key(&testing::SECRET);
        let mut entries = entries_of(&pack());
        for (name, data) in &mut entries {
            if name == SIGNATURE_ENTRY {
                data.truncate(63);
            }
        }
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::MalformedSignature {
                expected: 64,
                actual: 63
            })
        ));
    }

    #[test]
    fn a_perturbed_manifest_is_refused_as_non_canonical() {
        let key = signing_key(&testing::SECRET);
        let mut entries = entries_of(&pack());
        for (name, data) in &mut entries {
            if name == MANIFEST_ENTRY {
                data.insert(1, b' ');
            }
        }
        // Re-sign the perturbed bytes, so the refusal is canonicality itself
        // rather than the signature that would otherwise mask it.
        let perturbed = content_of(&entries, MANIFEST_ENTRY).to_vec();
        let signature = sign_manifest(&perturbed, &key).to_vec();
        for (name, data) in &mut entries {
            if name == SIGNATURE_ENTRY {
                data.clone_from(&signature);
            }
        }
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::NonCanonicalManifest)
        ));
    }

    #[test]
    fn an_unlisted_extra_entry_is_refused() {
        let key = signing_key(&testing::SECRET);
        let mut entries = entries_of(&pack());
        entries.push(("models/stowaway.lib".to_owned(), b"* extra\n".to_vec()));
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::UnlistedEntry(name)) if name == "models/stowaway.lib"
        ));
    }

    #[test]
    fn a_listed_but_absent_file_is_refused() {
        let key = signing_key(&testing::SECRET);
        let mut entries = entries_of(&pack());
        entries.retain(|(name, _)| name != "LICENSE");
        assert!(matches!(
            Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
            Err(PackError::MissingListedFile(name)) if name == "LICENSE"
        ));
    }

    #[test]
    fn a_missing_container_entry_is_refused() {
        let key = signing_key(&testing::SECRET);
        for reserved in RESERVED_ENTRIES {
            let mut entries = entries_of(&pack());
            entries.retain(|(name, _)| name != reserved);
            assert!(matches!(
                Pack::verify(&assemble(entries), &key.verifying_key(), &Limits::default()),
                Err(PackError::MissingReservedEntry(name)) if name == reserved
            ));
        }
    }

    #[test]
    fn the_injected_caps_refuse_an_otherwise_valid_pack() {
        let archive = pack();
        let public = signing_key(&testing::SECRET).verifying_key();
        assert!(matches!(
            Pack::verify(
                &archive,
                &public,
                &Limits {
                    max_archive_bytes: 64,
                    ..Limits::default()
                }
            ),
            Err(PackError::ArchiveTooLarge { limit: 64, .. })
        ));
        assert!(matches!(
            Pack::verify(
                &archive,
                &public,
                &Limits {
                    max_expanded_bytes: 8,
                    ..Limits::default()
                }
            ),
            Err(PackError::ExpandedTooLarge { limit: 8 })
        ));
        assert!(matches!(
            Pack::verify(
                &archive,
                &public,
                &Limits {
                    max_files: 2,
                    ..Limits::default()
                }
            ),
            Err(PackError::TooManyFiles { limit: 2, .. })
        ));
    }

    #[test]
    fn the_archive_subset_is_enforced_ahead_of_the_signature() {
        let public = signing_key(&testing::SECRET).verifying_key();

        let mut archive = pack();
        let central = central_offset(&archive);
        patch_u16(&mut archive, central + CENTRAL_FLAGS, 1 << 3);
        assert!(matches!(
            Pack::verify(&archive, &public, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(_))
        ));

        let mut archive = pack();
        let central = central_offset(&archive);
        patch_u16(&mut archive, central + CENTRAL_METHOD, 99);
        assert!(matches!(
            Pack::verify(&archive, &public, &Limits::default()),
            Err(PackError::UnsupportedArchiveFeature(_))
        ));

        // A stored-method archive reads successfully; it is refused only for
        // the container entries it does not carry.
        let stored = build_stored_archive("models/a.lib", b"stored");
        assert!(matches!(
            Pack::verify(&stored, &public, &Limits::default()),
            Err(PackError::MissingReservedEntry(_))
        ));
    }

    #[test]
    fn the_packer_refuses_reserved_and_traversal_paths() {
        let key = signing_key(&testing::SECRET);
        for reserved in RESERVED_ENTRIES {
            let files = vec![(reserved.to_owned(), b"x".to_vec())];
            assert!(matches!(
                build_pack(&files, testing::template(), &key),
                Err(PackError::ReservedFileName(_))
            ));
        }
        for path in ["../escape.lib", "a//b.lib", "a\\b.lib", "/abs.lib"] {
            let files = vec![(path.to_owned(), b"x".to_vec())];
            assert!(matches!(
                build_pack(&files, testing::template(), &key),
                Err(PackError::IllegalPath(_))
            ));
        }
    }

    #[test]
    fn the_packer_refuses_a_duplicated_input_path() {
        let key = signing_key(&testing::SECRET);
        let mut files = testing::files();
        let first = files[0].clone();
        files.push(first);
        assert!(matches!(
            build_pack(&files, testing::template(), &key),
            Err(PackError::DuplicateEntry(_))
        ));
    }

    #[test]
    fn the_packer_refuses_a_file_set_the_manifest_cannot_describe() {
        let key = signing_key(&testing::SECRET);
        let mut template = testing::template();
        template.parts[0].source.path = "models/absent.lib".to_owned();
        assert!(matches!(
            build_pack(&testing::files(), template, &key),
            Err(PackError::InvalidField { .. })
        ));
    }
}
