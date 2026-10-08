use super::*;
use crate::zip::deterministic_stored_zip;
use std::io::{Read as _, Write as _};

#[test]
fn source_directory_checks_do_not_confuse_member_contents_with_headers() {
    let content = b"PK\x01\x02 payload, not another directory entry";
    let bytes = deterministic_stored_zip(&[("first", content), ("second", b"ok")]).unwrap();
    let mut archive = open_unique_archive(&bytes, 2).unwrap();
    let mut restored = Vec::new();
    archive
        .by_name("first")
        .unwrap()
        .read_to_end(&mut restored)
        .unwrap();
    assert_eq!(restored, content);
    assert!(matches!(
        open_unique_archive(&bytes, 1),
        Err(ArchiveReadError::MemberCount {
            members: 2,
            limit: 1
        })
    ));
}

#[test]
fn source_directory_checks_support_deflate_zip64_comments_prefixes_and_trailers() {
    let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
    writer.set_comment("archive comment").unwrap();
    writer.set_raw_zip64_extensible_data_sector(Box::new([]));
    writer
        .start_file(
            "payload",
            zip::write::SimpleFileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .large_file(true),
        )
        .unwrap();
    writer.write_all(b"result samples").unwrap();
    let body = writer.finish().unwrap().into_inner();
    let mut bytes = b"container prefix".to_vec();
    bytes.extend(body);
    bytes.extend(b"trailing application bytes");
    let mut archive = open_unique_archive(&bytes, 1).unwrap();
    let mut contents = String::new();
    archive
        .by_name("payload")
        .unwrap()
        .read_to_string(&mut contents)
        .unwrap();
    assert_eq!(contents, "result samples");
    assert_eq!(archive.comment(), b"archive comment");
}

#[test]
fn undeclared_directory_entries_cannot_disappear_from_the_index() {
    let mut bytes = deterministic_stored_zip(&[("first", b"one"), ("second", b"two")]).unwrap();
    let footer = bytes.len() - 22;
    bytes[footer + 8..footer + 10].copy_from_slice(&1u16.to_le_bytes());
    bytes[footer + 10..footer + 12].copy_from_slice(&1u16.to_le_bytes());
    assert!(matches!(
        open_unique_archive(&bytes, 2),
        Err(ArchiveReadError::Zip(_))
    ));
    assert!(matches!(
        open_unique_archive(&bytes, 1),
        Err(ArchiveReadError::MemberCount {
            members: 2,
            limit: 1
        })
    ));
}
