//! Every result family the CLI can execute publishes the shared typed result
//! document under `-f json`, and every family it cannot execute is a typed
//! refusal.
//!
//! The sweep is exhaustive over `AnalysisResultKind`, so a new core result
//! family cannot be added without deciding — here, in a test that runs the
//! real binary — whether the CLI publishes it, and what identity, descriptors,
//! and units that artifact carries.

use rspice_core::execution::AnalysisResultKind;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

struct TestDirectory(PathBuf);

impl std::ops::Deref for TestDirectory {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn test_dir(tag: &str) -> TestDirectory {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let serial = NEXT.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "rspice_typed_documents_{}_{}_{}",
        std::process::id(),
        tag,
        serial
    ));
    std::fs::create_dir_all(&path).expect("create typed-document test directory");
    TestDirectory(path)
}

/// A small nonlinear divider that every family below can be run against.
const CIRCUIT: &str = "* typed result document coverage\n\
                       .param bias=0.5\n\
                       V1 in 0 DC {bias} AC 1 DISTOF1 1m 0 SIN({bias} 1m 1k) PORTNUM 1 Z0 50\n\
                       R1 in mid 1k\n\
                       D1 mid out DMOD\n\
                       R2 out 0 1k\n\
                       C1 out 0 1n\n\
                       V2 out2 0 DC 0 AC 0 PORTNUM 2 Z0 50\n\
                       R3 out out2 1k\n\
                       .model DMOD D(IS=1e-12)\n";

/// How one family is driven and where its artifact lands.
struct FamilyRun {
    /// Circuit this family is driven against; the shared one unless the
    /// family needs a different excitation.
    circuit: Option<&'static str>,
    /// Analysis cards appended to the circuit.
    cards: &'static str,
    /// Extra command-line flags.
    flags: &'static [&'static str],
    /// Artifact suffix after the requested output stem, if the family
    /// publishes under a namespace.
    artifact: &'static str,
    /// Canonical analysis identity the artifact must declare.
    analysis_tag: &'static str,
    /// A series the document must carry, with the unit it must declare.
    series: Option<(&'static str, &'static str)>,
    /// A named scalar the document must carry.
    scalar: Option<&'static str>,
}

/// How the CLI treats one result family.
enum FamilyCoverage {
    /// The family runs and publishes the shared typed document.
    Document(FamilyRun),
    /// The family runs, but one authored form cannot fill the shared payload,
    /// so that form is refused rather than published with evidence dropped.
    /// The refusal must contain each fragment.
    RefusedFormat {
        circuit: Option<&'static str>,
        cards: &'static str,
        flags: &'static [&'static str],
        fragments: &'static [&'static str],
    },
    /// The CLI has no execution route for the family at all.
    RefusedCard {
        cards: &'static str,
        fragments: &'static [&'static str],
    },
    /// The family publishes its own versioned artifact rather than the shared
    /// document, for a reason the registry records.
    OwnArtifact {
        cards: &'static str,
        artifact: &'static str,
        analysis: &'static str,
    },
}

fn coverage(kind: AnalysisResultKind) -> FamilyCoverage {
    match kind {
        AnalysisResultKind::OperatingPoint => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".OP\n",
            flags: &[],
            artifact: "",
            analysis_tag: "op-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::DcSweep => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".DC V1 0.4 0.6 0.1\n",
            flags: &[],
            artifact: "",
            analysis_tag: "dc-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::Ac => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".AC DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "ac-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::Transient => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".TRAN 100u 500u\n",
            flags: &[],
            artifact: "",
            analysis_tag: "tran-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::Noise => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".NOISE V(out) V1 DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "noise-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::SParameters => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".SP DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "sp-001",
            series: Some(("s(1,1)", "dimensionless")),
            scalar: None,
        }),
        // The `.SP DONOISE` covariance, its reference temperature, the 4kT
        // normalization, and the two-port noise figures have no home in the
        // shared S-parameter payload.
        AnalysisResultKind::PortNoise => FamilyCoverage::RefusedFormat {
            circuit: None,
            cards: ".SP DEC 2 1k 10k DONOISE\n",
            flags: &[],
            fragments: &["covariance", "two-port noise", "csv"],
        },
        // The Volterra products are normalized to the fundamental, so the
        // deck is driven directly into the nonlinearity: a node with no F1
        // response has no finite ratio and the run refuses rather than
        // publishing one.
        AnalysisResultKind::Distortion => FamilyCoverage::Document(FamilyRun {
            circuit: Some(
                "* distortion coverage\n\
                 V1 out 0 DC 0.5 DISTOF1 1m 0\n\
                 D1 out 0 DMOD\n\
                 .model DMOD D(IS=1e-12 N=1 CJO=0 TT=0)\n",
            ),
            cards: ".DISTO DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "disto-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::TransferFunction => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".TF V(out) V1\n",
            flags: &[],
            artifact: "",
            analysis_tag: "tf-001",
            series: None,
            scalar: Some("transfer_gain"),
        }),
        // Three cascaded poles, so the loop phase really reaches -180 degrees
        // and both Tian margins are finite. A loop with no such crossover has
        // an infinite margin that the shared document refuses to encode — a
        // core contract every surface shares, not a CLI gap.
        AnalysisResultKind::Stability => FamilyCoverage::Document(FamilyRun {
            circuit: Some(
                "* three-pole inverting loop\n\
                 E1 eo 0 ctrl 0 -1000\n\
                 VPROBE eo x 0\n\
                 R1 x n1 1k\n\
                 C1 n1 0 159.154943091895n\n\
                 R2 n1 n2 1k\n\
                 C2 n2 0 159.154943091895n\n\
                 R3 n2 ctrl 1k\n\
                 C3 ctrl 0 159.154943091895n\n",
            ),
            cards: ".STB DEC 20 10 10meg probe=VPROBE\n",
            flags: &[],
            artifact: "",
            analysis_tag: "stb-001",
            series: Some(("loop_gain", "dimensionless")),
            scalar: Some("phase_margin_degrees"),
        }),
        AnalysisResultKind::Sensitivity => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".SENS V(out) DC\n",
            flags: &[],
            artifact: "",
            analysis_tag: "sens-001",
            series: None,
            scalar: None,
        }),
        AnalysisResultKind::PoleZero => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".PZ in 0 out 0 vol pz\n",
            flags: &[],
            artifact: "",
            analysis_tag: "pz-001",
            series: None,
            scalar: None,
        }),
        AnalysisResultKind::Fourier => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".TRAN 10u 3m\n.FOUR 1k V(out)\n",
            flags: &[],
            artifact: "four-001",
            analysis_tag: "four-001",
            series: Some(("harmonic_magnitude", "volt")),
            scalar: Some("fundamental_frequency"),
        }),
        // `.FFT` keeps its own versioned bundle: it already carries instance
        // and coordinate identity plus the complete transform contract, and it
        // must publish atomically with its parent transient through the
        // two-member artifact pair.
        AnalysisResultKind::Fft => FamilyCoverage::OwnArtifact {
            cards: ".TRAN 10u 3m\n.FFT V(out)\n",
            artifact: "fft",
            analysis: "fft",
        },
        AnalysisResultKind::MonteCarlo => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".OP\n",
            flags: &["--monte-carlo", "4", "--seed", "7", "--mc-spread", "0.02"],
            artifact: "",
            analysis_tag: "mc-001",
            series: None,
            scalar: Some("completed_runs"),
        }),
        AnalysisResultKind::Pss => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".TRAN 10u 1m\n",
            flags: &["--pss-freq", "1k"],
            artifact: "",
            analysis_tag: "pss-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::HarmonicBalance => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            cards: ".HB 1k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "hb-001",
            series: Some(("v(out)", "volt")),
            scalar: Some("converged"),
        }),
        AnalysisResultKind::Pac => FamilyCoverage::RefusedCard {
            cards: ".HB 1k\n.PAC DEC 2 1k 10k INPUT=V1 OUT=V(out)\n",
            fragments: &[".PAC"],
        },
        AnalysisResultKind::PNoise => FamilyCoverage::RefusedCard {
            cards: ".HB 1k\n.PNOISE DEC 2 1k 10k OUT=V(out)\n",
            fragments: &[".PNOISE"],
        },
        AnalysisResultKind::Envelope => FamilyCoverage::RefusedCard {
            cards: ".HB 1k\n.ENVELOPE TSTOP=1m\n",
            fragments: &[".ENVELOPE"],
        },
    }
}

fn run(
    dir: &Path,
    circuit: Option<&str>,
    cards: &str,
    flags: &[&str],
    format: &str,
) -> (std::process::Output, PathBuf) {
    let deck = dir.join("deck.sp");
    let circuit = circuit.unwrap_or(CIRCUIT);
    std::fs::write(&deck, format!("{circuit}{cards}.END\n")).expect("write deck");
    let requested = dir.join(format!("result.{format}"));
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
    command.args([
        "--quiet",
        "run",
        deck.to_str().expect("UTF-8 deck path"),
        "-o",
        requested.to_str().expect("UTF-8 output path"),
        "-f",
        format,
    ]);
    command.args(flags);
    (command.output().expect("run rspice"), requested)
}

fn artifact_path(requested: &Path, suffix: &str) -> PathBuf {
    if suffix.is_empty() {
        return requested.to_path_buf();
    }
    let stem = requested
        .file_stem()
        .expect("output stem")
        .to_string_lossy();
    let extension = requested.extension().expect("output extension");
    requested.with_file_name(format!("{stem}.{suffix}.{}", extension.to_string_lossy()))
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap_or_else(|error| {
        panic!("read {}: {error}", path.display());
    }))
    .unwrap_or_else(|error| panic!("parse {}: {error}", path.display()))
}

fn assert_document(kind: AnalysisResultKind, run: &FamilyRun) {
    let dir = test_dir(kind.tag());
    let (output, requested) = run_family(&dir, run);
    assert!(
        output.status.success(),
        "{}: run failed:\n{}",
        kind.tag(),
        String::from_utf8_lossy(&output.stderr)
    );
    let path = artifact_path(&requested, run.artifact);
    let document = read_json(&path);

    assert_eq!(
        document["schema"],
        "rspice-analysis-result",
        "{}: artifact is not a shared result document",
        kind.tag()
    );
    assert_eq!(
        document["resultKind"],
        kind.tag(),
        "{}: document declares a different result family",
        kind.tag()
    );
    assert_eq!(
        document["analysis"]["tag"],
        run.analysis_tag,
        "{}: document declares a different analysis identity",
        kind.tag()
    );
    assert!(
        document["namespaces"]["output"] == run.analysis_tag,
        "{}: document does not name its own artifact namespace",
        kind.tag()
    );

    if let Some((name, unit)) = run.series {
        let signal = document["signals"]
            .as_array()
            .unwrap_or_else(|| panic!("{}: document has no signals array", kind.tag()))
            .iter()
            .find(|signal| {
                signal["descriptor"]["canonicalName"]
                    .as_str()
                    .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
            })
            .unwrap_or_else(|| {
                panic!(
                    "{}: document has no series '{name}' in {document:#}",
                    kind.tag()
                )
            });
        assert_eq!(
            signal["descriptor"]["unit"]["unit"],
            unit,
            "{}: series '{name}' declares the wrong unit",
            kind.tag()
        );
        assert!(
            !signal["descriptor"]["displayName"]
                .as_str()
                .unwrap_or_default()
                .is_empty(),
            "{}: series '{name}' has no display name",
            kind.tag()
        );
    }

    if let Some(name) = run.scalar {
        assert!(
            document["scalars"]
                .as_array()
                .unwrap_or_else(|| panic!("{}: document has no scalars array", kind.tag()))
                .iter()
                .any(|scalar| scalar["name"].as_str() == Some(name)),
            "{}: document has no scalar '{name}' in {document:#}",
            kind.tag()
        );
    }
}

fn run_family(dir: &Path, run: &FamilyRun) -> (std::process::Output, PathBuf) {
    crate::run(dir, run.circuit, run.cards, run.flags, "json")
}

/// HDF5 keys its section group by the analysis instance, not by the result
/// family, so two `.AC` cards in one deck cannot collide and a reader can tell
/// which card a group came from without reading the filename.
#[test]
fn hdf5_groups_are_keyed_by_analysis_identity() {
    let dir = test_dir("hdf5_identity");
    let (output, requested) = run(
        &dir,
        None,
        ".AC DEC 2 1k 10k\n.AC DEC 2 10k 100k\n",
        &[],
        "hdf5",
    );
    assert!(
        output.status.success(),
        "repeated .AC run failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );

    for tag in ["ac-001", "ac-002"] {
        let path = artifact_path(&requested, tag);
        let bytes = std::fs::read(&path).unwrap_or_else(|error| {
            panic!("read {}: {error}", path.display());
        });
        assert!(
            bytes
                .windows(tag.len())
                .any(|window| window == tag.as_bytes()),
            "{tag}: the analysis identity does not appear in its own HDF5 document"
        );
        // The section is still found on read-back, which it can only be by its
        // declared `section_type` rather than by a hard-coded group name.
        let converted = dir.join(format!("{tag}.csv"));
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "--quiet",
                "convert",
                "--to",
                "csv",
                path.to_str().expect("UTF-8 HDF5 path"),
                converted.to_str().expect("UTF-8 converted path"),
            ])
            .output()
            .expect("convert HDF5 artifact");
        assert!(
            output.status.success(),
            "{tag}: an identity-keyed HDF5 group could not be read back:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = std::fs::read_to_string(&converted).expect("read converted CSV");
        assert!(
            text.lines()
                .next()
                .is_some_and(|header| header.contains("frequency")),
            "{tag}: the converted AC table lost its frequency axis"
        );
    }
}

#[test]
fn every_runnable_family_publishes_a_typed_document_and_the_rest_are_typed_refusals() {
    for kind in AnalysisResultKind::ALL {
        match coverage(kind) {
            FamilyCoverage::Document(family) => assert_document(kind, &family),
            FamilyCoverage::RefusedFormat {
                circuit,
                cards,
                flags,
                fragments,
            } => {
                let dir = test_dir(kind.tag());
                let (output, requested) = run(&dir, circuit, cards, flags, "json");
                assert!(
                    !output.status.success(),
                    "{}: a format that cannot carry this result published anyway",
                    kind.tag()
                );
                let stderr = String::from_utf8_lossy(&output.stderr);
                for fragment in fragments {
                    assert!(
                        stderr.contains(fragment),
                        "{}: refusal does not mention {fragment:?}: {stderr}",
                        kind.tag()
                    );
                }
                assert!(
                    !requested.exists(),
                    "{}: the refused run published an artifact",
                    kind.tag()
                );

                // The same deck exports completely in a format that can hold
                // it, so the refusal is about representation, not capability.
                let (output, requested) = run(&dir, circuit, cards, flags, "csv");
                assert!(
                    output.status.success(),
                    "{}: the flat export failed too:\n{}",
                    kind.tag(),
                    String::from_utf8_lossy(&output.stderr)
                );
                assert!(requested.exists() || artifact_path(&requested, "sp-001").exists());
            }
            FamilyCoverage::RefusedCard { cards, fragments } => {
                let dir = test_dir(kind.tag());
                let (output, requested) = run(&dir, None, cards, &[], "json");
                assert!(
                    !output.status.success(),
                    "{}: an unroutable card was accepted",
                    kind.tag()
                );
                let stderr = String::from_utf8_lossy(&output.stderr);
                for fragment in fragments {
                    assert!(
                        stderr.contains(fragment),
                        "{}: refusal does not name the card {fragment:?}: {stderr}",
                        kind.tag()
                    );
                }
                assert!(
                    !requested.exists(),
                    "{}: a refused card published an artifact",
                    kind.tag()
                );
            }
            FamilyCoverage::OwnArtifact {
                cards,
                artifact,
                analysis,
            } => {
                let dir = test_dir(kind.tag());
                let (output, requested) = run(&dir, None, cards, &[], "json");
                assert!(
                    output.status.success(),
                    "{}: run failed:\n{}",
                    kind.tag(),
                    String::from_utf8_lossy(&output.stderr)
                );
                let document = read_json(&artifact_path(&requested, artifact));
                assert_eq!(document["analysis"], analysis, "{}", kind.tag());
                // Its own schema is versioned and carries instance identity,
                // which is what the shared document exists to guarantee.
                assert!(
                    document["schema_version"].as_u64().is_some(),
                    "{}: the family's own artifact is unversioned",
                    kind.tag()
                );
                assert!(
                    document["results"][0]["analysis_id"]
                        .as_str()
                        .is_some_and(|tag| tag.starts_with("fft-")),
                    "{}: the family's own artifact carries no instance identity",
                    kind.tag()
                );
                assert_eq!(document["parent_analysis_id"], "tran-001", "{}", kind.tag());
            }
        }
    }
}
