//! Native bundle byte contracts and bounded independent-reader regression.

use super::reader::read_zip_member;
use super::*;
use std::io::{Cursor, Write as _};

const MAX_RESULT_DATASET_BYTES: u64 = 64 * 1024 * 1024;

fn limits() -> NativeBundleReadLimits {
    NativeBundleReadLimits {
        max_members: 1_024,
        max_expanded_bytes: MAX_RESULT_DATASET_BYTES,
        max_member_bytes: MAX_RESULT_DATASET_BYTES,
    }
}

fn zip_bytes(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);
    for (name, contents) in entries {
        writer.start_file(*name, options).expect("start ZIP member");
        writer.write_all(contents).expect("write ZIP member");
    }
    writer.finish().expect("finish ZIP").into_inner()
}

#[test]
fn native_export_schema_is_deterministic_and_round_trips_real_and_complex() {
    let coordinate = [1.0e3, 2.0e3, 4.0e3];
    let real_values = [0.25, 0.5, 1.0];
    let complex_real = [1.0, -2.0, 0.5];
    let complex_imag = [0.125, 0.25, -0.75];
    let dataset = NativeBundleDataset {
        analysis: NativeBundleAnalysis::Ac,
        coordinate_name: "frequency",
        coordinate: &coordinate,
        signals: vec![
            NativeBundleSignal {
                name: "gain",
                unit: None,
                values: NativeBundleSignalValues::Real(&real_values),
            },
            NativeBundleSignal {
                name: "V(out)",
                unit: Some("V"),
                values: NativeBundleSignalValues::Complex {
                    real: &complex_real,
                    imag: &complex_imag,
                },
            },
        ],
    };

    for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
        let bytes = encode_native_bundle(kind, &dataset, MAX_RESULT_DATASET_BYTES)
            .expect("native bundle encode");
        assert_eq!(
            bytes,
            encode_native_bundle(kind, &dataset, MAX_RESULT_DATASET_BYTES)
                .expect("repeat deterministic encode")
        );

        let mut archive = zip::ZipArchive::new(Cursor::new(bytes.as_slice())).unwrap();
        assert_eq!(archive.len(), 2);
        let manifest_bytes =
            read_zip_member(&mut archive, "manifest.json", MAX_RESULT_DATASET_BYTES).unwrap();
        let dataset_bytes =
            read_zip_member(&mut archive, "dataset.json", MAX_RESULT_DATASET_BYTES).unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(&manifest_bytes).unwrap();
        let document: serde_json::Value = serde_json::from_slice(&dataset_bytes).unwrap();
        assert_eq!(manifest["schema"], kind.manifest_schema());
        assert_eq!(manifest["dataset_member"], "dataset.json");
        assert_eq!(document["schema"], "rspice-waveform-dataset/1");
        assert_eq!(document["analysis"], "ac");
        assert_eq!(document["signals"][0]["values"][1], 0.5);
        assert_eq!(document["signals"][1]["real"][1], -2.0);
        assert_eq!(document["signals"][1]["imag"][2], -0.75);
        use sha2::Digest as _;
        assert_eq!(
            manifest["dataset_sha256"],
            format!("{:x}", sha2::Sha256::digest(&dataset_bytes))
        );

        let parsed =
            decode_native_bundle(&bytes, kind, limits()).expect("exporter/importer round-trip");
        assert_eq!(parsed.analysis, "ac");
        assert_eq!(parsed.coordinate.name, "frequency");
        assert_eq!(parsed.coordinate.values, coordinate);
        assert_eq!(parsed.signals.len(), 2);
        assert_eq!(parsed.signals[0].name, "gain");
        assert_eq!(
            parsed.signals[0].values.as_deref(),
            Some(real_values.as_slice())
        );
        assert_eq!(parsed.signals[1].name, "V(out)");
        assert_eq!(
            parsed.signals[1].real.as_deref(),
            Some(complex_real.as_slice())
        );
        assert_eq!(
            parsed.signals[1].imag.as_deref(),
            Some(complex_imag.as_slice())
        );
        assert_eq!(parsed.signals[1].unit.as_deref(), Some("V"));

        for bounds in [
            NativeBundleReadLimits {
                max_members: 1,
                ..limits()
            },
            NativeBundleReadLimits {
                max_expanded_bytes: 0,
                ..limits()
            },
            NativeBundleReadLimits {
                max_member_bytes: 0,
                ..limits()
            },
        ] {
            assert!(matches!(
                decode_native_bundle(&bytes, kind, bounds),
                Err(NativeBundleError::InvalidData(_))
            ));
        }
        assert!(matches!(
            encode_native_bundle(kind, &dataset, 1),
            Err(NativeBundleError::InvalidData(_))
        ));
        let malformed = decode_native_bundle(&[], kind, limits()).unwrap_err();
        assert!(matches!(malformed, NativeBundleError::Zip { .. }));
        assert!(std::error::Error::source(&malformed).is_some());
        decode_native_bundle(
            &bytes,
            kind,
            NativeBundleReadLimits {
                max_member_bytes: u64::MAX,
                ..limits()
            },
        )
        .expect("a representable maximum bound does not overflow");

        let mut tampered_manifest = manifest;
        tampered_manifest["dataset_sha256"] = serde_json::Value::String("00".repeat(32));
        let tampered_manifest = serde_json::to_vec(&tampered_manifest).unwrap();
        let tampered = zip_bytes(&[
            ("manifest.json", &tampered_manifest),
            ("dataset.json", &dataset_bytes),
        ]);
        let error = decode_native_bundle(&tampered, kind, limits()).expect_err("digest tamper");
        assert!(matches!(error, NativeBundleError::InvalidData(_)));
        assert!(error.to_string().contains("SHA-256"), "{error}");
    }
}

#[test]
fn native_decode_preserves_record_names_in_json_errors() {
    let valid_coordinate = serde_json::json!({"name": "time", "values": [0.0, 1.0]});
    for (document, record) in [
        (serde_json::Value::Null, "NativeDataset"),
        (
            serde_json::json!({"schema": "rspice-waveform-dataset/1", "analysis": "transient", "coordinate": null, "signals": []}),
            "NativeCoordinate",
        ),
        (
            serde_json::json!({"schema": "rspice-waveform-dataset/1", "analysis": "transient", "coordinate": valid_coordinate, "signals": [null]}),
            "NativeSignal",
        ),
    ] {
        let dataset = serde_json::to_vec(&document).unwrap();
        use sha2::Digest as _;
        let manifest = serde_json::to_vec(&serde_json::json!({
            "schema": "rspice-result-bundle/1",
            "dataset_member": "dataset.json",
            "dataset_sha256": format!("{:x}", sha2::Sha256::digest(&dataset)),
        }))
        .unwrap();
        let bytes = zip_bytes(&[("manifest.json", &manifest), ("dataset.json", &dataset)]);
        let error = decode_native_bundle(&bytes, NativeBundleKind::Result, limits()).unwrap_err();
        assert!(matches!(error, NativeBundleError::Json { .. }));
        assert!(std::error::Error::source(&error).is_some());
        assert!(
            error.to_string().starts_with(&format!(
                "dataset.json is invalid: invalid type: null, expected struct {record} at line "
            )),
            "{error}"
        );
    }
}
