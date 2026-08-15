use std::collections::BTreeMap;

use crate::manifest::{
    FileEntry, License, MANIFEST_SCHEMA, Manifest, ManifestTemplate, PackIdentity, Part, PartKind,
    Requires, SourceRef, Symbol,
};
use crate::signing::sha256_hex;

/// Fixed secret key material. Deterministic fixtures are what let the tests
/// assert byte-identical archives.
pub(crate) const SECRET: [u8; 32] = [0x2a; 32];

pub(crate) const OPAMP_LIBRARY: &[u8] =
    b"* RSpice op-amp models\n.subckt TL072 INP INN VCC VEE OUT\n.ends\n";
pub(crate) const LICENSE_TEXT: &[u8] = b"LicenseRef-RSpice-Models\n";

pub(crate) fn files() -> Vec<(String, Vec<u8>)> {
    vec![
        ("models/opamps.lib".to_owned(), OPAMP_LIBRARY.to_vec()),
        ("LICENSE".to_owned(), LICENSE_TEXT.to_vec()),
    ]
}

pub(crate) fn template() -> ManifestTemplate {
    ManifestTemplate {
        pack: PackIdentity {
            id: "rspice-opamps".to_owned(),
            version: "1.0.0".to_owned(),
            name: "RSpice Op-Amps".to_owned(),
            category: "ic-analog".to_owned(),
        },
        license: License {
            spdx: "LicenseRef-RSpice-Models".to_owned(),
        },
        requires: Requires {
            capabilities: vec![
                "subckt".to_owned(),
                "bjt-gp".to_owned(),
                "diode-level1".to_owned(),
            ],
        },
        parts: vec![
            Part {
                id: "TL072".to_owned(),
                kind: PartKind::Subckt,
                device: "opamp".to_owned(),
                aliases: vec!["TL072CP".to_owned()],
                source: SourceRef {
                    path: "models/opamps.lib".to_owned(),
                    line: 214,
                },
                terminals: vec![
                    "INP".to_owned(),
                    "INN".to_owned(),
                    "VCC".to_owned(),
                    "VEE".to_owned(),
                    "OUT".to_owned(),
                ],
                symbol: Some(Symbol {
                    reference: "engine:opamp-5t".to_owned(),
                    pins: BTreeMap::from([
                        ("INP".to_owned(), "in+".to_owned()),
                        ("INN".to_owned(), "in-".to_owned()),
                        ("VCC".to_owned(), "v+".to_owned()),
                        ("VEE".to_owned(), "v-".to_owned()),
                        ("OUT".to_owned(), "out".to_owned()),
                    ]),
                }),
            },
            Part {
                id: "1N4148".to_owned(),
                kind: PartKind::Model,
                device: "diode".to_owned(),
                aliases: Vec::new(),
                source: SourceRef {
                    path: "models/opamps.lib".to_owned(),
                    line: 402,
                },
                terminals: vec!["A".to_owned(), "K".to_owned()],
                symbol: None,
            },
        ],
    }
}

pub(crate) fn manifest() -> Manifest {
    let template = template();
    let mut entries: Vec<(String, Vec<u8>)> = files();
    entries.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    Manifest {
        schema: MANIFEST_SCHEMA,
        pack: template.pack,
        license: template.license,
        requires: template.requires,
        files: entries
            .iter()
            .map(|(path, content)| FileEntry {
                path: path.clone(),
                length: content.len() as u64,
                sha256: sha256_hex(content),
            })
            .collect(),
        parts: template.parts,
    }
}
