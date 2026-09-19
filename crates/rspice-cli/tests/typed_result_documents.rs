//! Every result family the CLI executes publishes the shared typed result
//! document under `-f json`.
//!
//! The sweep is exhaustive over `AnalysisResultKind`, so a new core result
//! family cannot be added without deciding — here, in a test that runs the
//! real binary — whether the CLI publishes it, and what identity, descriptors,
//! and units that artifact carries.

mod common;

use common::{read_json, test_dir};

use rspice_core::execution::AnalysisResultKind;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    /// Spectre `.scs` library written beside the deck as `statistics.scs`,
    /// for a family whose deck `.include`s one. A statistical card reads the
    /// design's own `statistics` block, and the include expander is what
    /// lowers that block into the executable plan, so the family is driven
    /// through the same two files a user's deck is.
    library: Option<&'static str>,
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
            library: None,
            cards: ".OP\n",
            flags: &[],
            artifact: "",
            analysis_tag: "op-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::DcSweep => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".DC V1 0.4 0.6 0.1\n",
            flags: &[],
            artifact: "",
            analysis_tag: "dc-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::Ac => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".AC DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "ac-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::Transient => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".TRAN 100u 500u\n",
            flags: &[],
            artifact: "",
            analysis_tag: "tran-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::Noise => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".NOISE V(out) V1 DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "noise-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::SParameters => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".SP DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "sp-001",
            series: Some(("s(1,1)", "dimensionless")),
            scalar: None,
        }),
        // Port noise is the `.SP DONOISE` card's second result. It shares the
        // card's identity and publishes as its own document beside the
        // scattering one, carrying the covariance, its reference temperature,
        // the 4kT normalization and the two-port figures.
        AnalysisResultKind::PortNoise => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".SP DEC 2 1k 10k DONOISE\n",
            flags: &[],
            artifact: "port-noise",
            analysis_tag: "sp-001",
            series: Some(("cy(1,1)", "custom")),
            scalar: None,
        }),
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
            library: None,
            cards: ".DISTO DEC 2 1k 10k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "disto-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::TransferFunction => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
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
            library: None,
            cards: ".STB DEC 20 10 10meg probe=VPROBE\n",
            flags: &[],
            artifact: "",
            analysis_tag: "stb-001",
            series: Some(("loop_gain", "dimensionless")),
            scalar: Some("phase_margin_degrees"),
        }),
        AnalysisResultKind::Sensitivity => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".SENS V(out) DC\n",
            flags: &[],
            artifact: "",
            analysis_tag: "sens-001",
            series: None,
            scalar: None,
        }),
        AnalysisResultKind::PoleZero => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".PZ in 0 out 0 vol pz\n",
            flags: &[],
            artifact: "",
            analysis_tag: "pz-001",
            series: None,
            scalar: None,
        }),
        AnalysisResultKind::Fourier => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
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
            library: None,
            cards: ".OP\n",
            flags: &["--monte-carlo", "4", "--seed", "7", "--mc-spread", "0.02"],
            artifact: "",
            analysis_tag: "mc-001",
            series: None,
            scalar: Some("completed_runs"),
        }),
        AnalysisResultKind::Pss => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".PSS FUND=1k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "pss-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        AnalysisResultKind::HarmonicBalance => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".HB 1k\n",
            flags: &[],
            artifact: "",
            analysis_tag: "hb-001",
            series: Some(("v(out)", "volt")),
            scalar: Some("converged"),
        }),
        // The periodic small-signal families linearize around the carrier the
        // plan binds them to, so each deck authors the carrier first. The
        // sideband span is kept inside the carrier's harmonic capacity, which
        // the core refuses to exceed rather than truncate.
        AnalysisResultKind::Pac => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".HB 1k\n.PAC DEC 2 1k 10k INPUT=V1 OUT=V(out) MAXSIDEBAND=1\n",
            flags: &[],
            artifact: "pac-001",
            analysis_tag: "pac-001",
            series: Some(("v(out)", "volt")),
            scalar: Some("residual_norm"),
        }),
        // `.PXF` reads one conversion path out of the same solve `.PAC` runs,
        // so the sideband pair it names has to be inside the depth it states.
        // The transfer is volts out per unit of the drive's own unit, which
        // the artifact does not record, so the checked series is the converted
        // output frequency, whose unit is not in doubt.
        AnalysisResultKind::Pxf => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".HB 1k\n.PXF DEC 2 1k 10k INPUT=V1 OUT=V(out) MAXSIDEBAND=1\n",
            flags: &[],
            artifact: "pxf-001",
            analysis_tag: "pxf-001",
            series: Some(("output_frequency", "hertz")),
            scalar: Some("peak_gain_db"),
        }),
        // Floquet stability needs a loop probe to read the orbit at, and a
        // probe is an inductor current: the shared divider has no inductor, so
        // this family is driven against a series-RLC. The carrier is a
        // shooting `.PSS` because a monodromy matrix exists nowhere else.
        AnalysisResultKind::Pstb => FamilyCoverage::Document(FamilyRun {
            circuit: Some(
                "* periodic stability coverage\n\
                 VIN in 0 SIN(0 1 1meg)\n\
                 R1 in a 50\n\
                 L1 a out 10u\n\
                 C1 out 0 1n\n",
            ),
            library: None,
            cards: ".PSS FUND=1meg HARMS=8 POINTS=64 TSTABPERIODS=2\n\
                    .PSTB PROBE=L1 MAXHARM=4\n",
            flags: &[],
            artifact: "pstb-001",
            analysis_tag: "pstb-001",
            series: Some(("multiplier_magnitude", "dimensionless")),
            scalar: Some("max_multiplier_magnitude"),
        }),
        AnalysisResultKind::PNoise => FamilyCoverage::Document(FamilyRun {
            circuit: None,
            library: None,
            cards: ".HB 1k\n.PNOISE DEC 2 1k 10k OUT=V(out) INPUT=V1 MAXSIDEBAND=1\n",
            flags: &[],
            artifact: "pnoise-001",
            analysis_tag: "pnoise-001",
            // A driven run reports an absolute output PSD in V^2/Hz, which the
            // shared unit vocabulary carries as a named custom unit.
            series: Some(("output_noise", "custom")),
            scalar: Some("carrier_frequency"),
        }),
        // Envelope following publishes the continued slow-time trajectory, so
        // its series are the transient's and its carrier lives in the payload
        // rather than in a named scalar. Its exact carrier initializer covers
        // linear R/C networks and independent sources, so the shared nonlinear
        // deck is replaced by one it is defined for rather than accepting an
        // approximated continuation.
        AnalysisResultKind::Envelope => FamilyCoverage::Document(FamilyRun {
            circuit: Some(
                "* envelope coverage\n\
                 V1 in 0 SIN(0 1 1k)\n\
                 R1 in out 1k\n\
                 C1 out 0 159.154943091895n\n",
            ),
            library: None,
            cards: ".HB 1k\n.ENVELOPE TSTOP=1m MAXSTEP=0.1m\n",
            flags: &[],
            artifact: "env-001",
            analysis_tag: "env-001",
            series: Some(("v(out)", "volt")),
            scalar: None,
        }),
        // DC mismatch has no default spread to fall back on: every sigma comes
        // from the design's own `statistics` block, so this family is driven
        // against a resistor divider whose two resistances are varied by a
        // Spectre library the deck includes. The result is five named scalars
        // and a ranked contributor table, so there is no series to check.
        AnalysisResultKind::DcMatch => FamilyCoverage::Document(FamilyRun {
            circuit: Some(
                "* DC mismatch coverage\n\
                 .include \"statistics.scs\"\n\
                 V1 in 0 1\n\
                 R1 in out {r1v}\n\
                 R2 out 0 {r2v}\n",
            ),
            library: Some(
                "// Resistor divider mismatch, declared the way a PDK does.\n\
                 parameters r1v=1000 r2v=2000\n\
                 statistics {\n\
                  mismatch {\n\
                   vary r1v dist=gauss std=10\n\
                   vary r2v dist=gauss std=10\n\
                  }\n\
                 }\n",
            ),
            cards: ".DCMATCH OUT=V(out) CONTRIBUTORS=0 SIGMA=3\n",
            flags: &[],
            artifact: "",
            analysis_tag: "dcmatch-001",
            series: None,
            scalar: Some("sigma_total"),
        }),
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
    if let Some(library) = run.library {
        std::fs::write(dir.join("statistics.scs"), library)
            .expect("write the deck's Spectre statistics library");
    }
    crate::run(dir, run.circuit, run.cards, run.flags, "json")
}

/// A `.HB` card that states its own harmonic count needs no `.OPTIONS` line
/// beside it: `rspice run` solves the spectrum the card names.
///
/// Five harmonics and DC are six components per node, and the frequency axis
/// is the fundamental's multiples, so the count on the card is visible in the
/// published document rather than only in the solver's configuration.
#[test]
fn the_command_line_solves_the_harmonics_an_hb_card_names() {
    let dir = test_dir("hb_card_harmonics");
    let (output, requested) = run(&dir, None, ".HB 1k HARMS=5\n", &[], "json");
    assert!(
        output.status.success(),
        "an .HB card carrying HARMS= failed to run:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = read_json(&requested);

    let harmonics = document["scalars"]
        .as_array()
        .expect("the HB document has scalars")
        .iter()
        .find(|scalar| scalar["name"].as_str() == Some("harmonic_count"))
        .unwrap_or_else(|| panic!("no harmonic_count scalar in {document:#}"));
    assert_eq!(
        harmonics["value"]["value"], 5,
        "the card's HARMS= is the harmonic count that was solved"
    );

    let frequency = document["axes"]
        .as_array()
        .expect("the HB document has axes")
        .iter()
        .find(|axis| axis["name"].as_str() == Some("frequency"))
        .unwrap_or_else(|| panic!("no frequency axis in {document:#}"));
    let values = frequency["values"]["values"]
        .as_array()
        .expect("the frequency axis carries its coordinates");
    assert_eq!(values.len(), 6, "DC and five harmonics are six components");
    assert_eq!(values[0].as_f64(), Some(0.0));
    assert_eq!(values[1].as_f64(), Some(1.0e3));
    assert_eq!(values[5].as_f64(), Some(5.0e3));
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
fn every_result_family_publishes_a_typed_document() {
    for kind in AnalysisResultKind::ALL {
        match coverage(kind) {
            FamilyCoverage::Document(family) => assert_document(kind, &family),
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

/// The AC form of `.SENS` publishes the same shared document the DC form
/// does: the payload carries a complex derivative trace per parameter beside
/// the operating-point entries, so a sweep no longer has to be exported flat.
#[test]
fn ac_sensitivity_publishes_the_shared_document_with_its_frequency_traces() {
    let dir = test_dir("sens_ac");
    let (output, requested) = run(&dir, None, ".SENS V(out) AC DEC 2 1k 10k\n", &[], "json");
    assert!(
        output.status.success(),
        "AC .SENS failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = read_json(&requested);
    assert_eq!(document["resultKind"], "sensitivity");
    assert_eq!(document["analysis"]["tag"], "sens-001");
    let payload = &document["payload"];
    assert!(
        payload["entries"]
            .as_array()
            .expect("DC entry list")
            .is_empty(),
        "an AC study must not claim operating-point derivatives"
    );
    let ac_entries = payload["acEntries"].as_array().expect("AC entry list");
    assert!(
        !ac_entries.is_empty(),
        "the AC sweep published no derivative traces: {document:#}"
    );
    let axis = document["axes"][0]["values"]["values"]
        .as_array()
        .expect("frequency axis");
    for entry in ac_entries {
        for trace in ["absolute", "normalized", "magnitude", "phase"] {
            assert_eq!(
                entry[trace].as_array().map(Vec::len),
                Some(axis.len()),
                "trace '{trace}' of {entry:#} does not cover the frequency axis"
            );
        }
    }
}

/// The `--sens-param` probe differentiates against a netlist parameter rather
/// than a device instance, and the shared payload says so: the entry is tagged
/// as a parameter sensitivity and carries the nominal value and the normalized
/// derivative, not just the raw number.
#[test]
fn the_parameter_sensitivity_probe_publishes_the_shared_document() {
    let dir = test_dir("sens_param");
    let (output, requested) = run(
        &dir,
        None,
        "",
        &["--sens-output", "out", "--sens-param", "bias"],
        "json",
    );
    assert!(
        output.status.success(),
        "--sens-param failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = read_json(&requested);
    assert_eq!(document["resultKind"], "sensitivity");
    let entries = document["payload"]["entries"]
        .as_array()
        .expect("entry list");
    assert_eq!(entries.len(), 1, "the probe computes one derivative");
    assert_eq!(entries[0]["parameter"], "BIAS");
    assert_eq!(
        entries[0]["vectorName"], "PARAM:BIAS",
        "the probe names its row the way a .SENS card names the same parameter"
    );
    assert_eq!(entries[0]["elementKind"], "parameter");
    assert!(
        entries[0]["normalized"]
            .as_f64()
            .is_some_and(f64::is_finite),
        "the probe published no normalized derivative: {document:#}"
    );
    assert!(
        document["scalars"]
            .as_array()
            .expect("scalars")
            .iter()
            .any(|scalar| scalar["name"] == "output_value"),
        "the normalized derivative has no operating point to be relative to"
    );
}

/// A `.SENS` card that names the design parameters reports them on the command
/// line, beside the device rows of the same study, under the engine's own
/// spelling. `I(V1) = -drive/R1` gives `d/d(drive) = -1/R1` and
/// `d/d(R1) = +drive/R1^2` in closed form.
#[test]
fn the_command_line_reports_the_design_parameters_a_sens_card_names() {
    let dir = test_dir("sens_design_parameters");
    let (output, requested) = run(
        &dir,
        Some(
            "* design parameter sensitivity\n\
             .param drive=2\n\
             V1 out 0 {drive}\n\
             R1 out 0 2\n",
        ),
        ".SENS I(V1) R1 PARAM:*\n",
        &[],
        "json",
    );
    assert!(
        output.status.success(),
        ".SENS with a PARAM: filter failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document = read_json(&requested);
    assert_eq!(document["resultKind"], "sensitivity");
    let entries = document["payload"]["entries"]
        .as_array()
        .expect("entry list");
    let entry = |name: &str| {
        entries
            .iter()
            .find(|entry| entry["vectorName"] == name)
            .unwrap_or_else(|| panic!("no {name} row in {document:#}"))
    };
    assert_eq!(entries.len(), 2, "one device row and one design row");
    let design = entry("PARAM:DRIVE");
    assert_eq!(design["elementKind"], "parameter");
    assert_eq!(design["nominalValue"].as_f64(), Some(2.0));
    let derivative = design["absolute"].as_f64().expect("design derivative");
    assert!(
        (derivative + 0.5).abs() < 1e-9,
        "dI(V1)/d(drive) = -1/R1 = -0.5, got {derivative}"
    );
    let device = entry("R1")["absolute"].as_f64().expect("device derivative");
    assert!(
        (device - 0.5).abs() < 1e-8,
        "dI(V1)/dR1 = drive/R1^2 = +0.5, got {device}"
    );
}

/// `.FOUR` integrates the periods and the end time its card names, and the
/// published document carries those numbers — the window is not a Studio-only
/// setting that the command line quietly ignores.
///
/// A decaying exponential source makes the window observable in one number:
/// the mean of `exp(-t/tau)` over `[t1 - k/f, t1]` is
/// `tau (exp(-t0/tau) - exp(-t1/tau)) / (t1 - t0)`, which is 0.0314714 over one
/// 1 ms period ending at 4 ms and 0.0585099 over two — the same closed form
/// core's `the_window_ends_where_the_card_says` oracle checks.
#[test]
fn the_command_line_integrates_the_periods_a_four_card_names() {
    const CIRCUIT: &str = "* windowed Fourier\n\
                           V1 out 0 EXP(1 0 0 1m 1 1m)\n\
                           R1 out 0 1k\n";
    const TAU: f64 = 1e-3;

    let mean =
        |start: f64, stop: f64| TAU * ((-start / TAU).exp() - (-stop / TAU).exp()) / (stop - start);
    let expected = [mean(3e-3, 4e-3), mean(2e-3, 4e-3)];

    let mut measured = Vec::new();
    for periods in [1_usize, 2] {
        let dir = test_dir(&format!("four_window_{periods}"));
        let (output, requested) = run(
            &dir,
            Some(CIRCUIT),
            &format!(".TRAN 1u 5m 0 1u\n.FOUR 1k 4 V(out) PERIODS={periods} TO=4m\n"),
            &[],
            "json",
        );
        assert!(
            output.status.success(),
            "PERIODS={periods} failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let document = read_json(&artifact_path(&requested, "four-001"));
        measured.push(
            document["scalars"]
                .as_array()
                .expect("scalar list")
                .iter()
                .find(|scalar| scalar["name"].as_str() == Some("dc_component"))
                .and_then(|scalar| scalar["value"]["value"].as_f64())
                .unwrap_or_else(|| panic!("no dc_component in {document:#}")),
        );
    }

    // Tolerance 1e-3: what the published record carries is the transient's own
    // output trajectory, whose interpolation between accepted steps is itself
    // second order and contributes about 3e-5 here. The quadrature is proven
    // to 1e-8 against the same closed form by core's own oracle; what this
    // test decides is which interval was integrated, and the two intervals are
    // 0.027 apart, 27 times the tolerance.
    for (periods, (dc, want)) in measured.iter().zip(expected).enumerate() {
        assert!(
            (dc - want).abs() < 1e-3,
            "PERIODS={}: the closed-form mean is {want}, the document says {dc}",
            periods + 1
        );
    }
    let separation = measured[1] - measured[0];
    let predicted = expected[1] - expected[0];
    assert!(
        (separation - predicted).abs() < 1e-3,
        "two periods must exceed one by {predicted}, the documents differ by {separation}"
    );
}

#[test]
fn zero_output_sensitivity_reports_unavailability_in_console_and_json() {
    for (label, clause, entries) in [("dc", "", "entries"), ("ac", " AC LIN 2 1 2", "acEntries")] {
        let dir = test_dir(&format!("sens_zero_{label}"));
        let deck = dir.join("deck.sp");
        let artifact = dir.join("result.json");
        std::fs::write(
            &deck,
            format!("Zero output\nV1 out 0 DC 0 AC 0\nR1 out 0 1\n.sens V(out){clause}\n.end\n"),
        )
        .unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--verbose", "run"])
            .arg(&deck)
            .arg("-o")
            .arg(&artifact)
            .args(["-f", "json"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let console = String::from_utf8_lossy(&output.stdout);
        assert!(console.contains("unavailable"), "{console}");
        let document = read_json(&artifact);
        for entry in document["payload"][entries].as_array().unwrap() {
            let normalized = &entry["normalized"];
            let normalized = if label == "ac" {
                &normalized[0]
            } else {
                normalized
            };
            assert_eq!(normalized["unavailable"], "zero-output");
        }
    }
}
