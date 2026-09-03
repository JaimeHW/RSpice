//! Where one artifact goes and which canonical identity it publishes under.
//!
//! Every namespace component an artifact carries is decided here and nowhere
//! else, so a deck can never publish two results to one path: the analysis
//! instance the planner minted, the coordinate of a run axis, the run label of
//! an `.ALTER` or `.DATA` variant, and the child component a second document
//! published beside its parent takes.

// This module was split out of `run.rs` and still works against the run
// command's own context, errors, and helpers, so it takes the parent's
// imports rather than restating them.
use super::*;

pub(super) fn analysis_output_tag(analysis: &AnalysisCommand) -> Option<&'static str> {
    match analysis {
        AnalysisCommand::Op => Some("op"),
        AnalysisCommand::Dc { .. } => Some("dc"),
        AnalysisCommand::Ac { .. } | AnalysisCommand::AcData { .. } => Some("ac"),
        AnalysisCommand::Tran { .. } => Some("tran"),
        AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. } => Some("noise"),
        AnalysisCommand::Sp { .. } => Some("sp"),
        AnalysisCommand::Stb { .. } => Some("stb"),
        AnalysisCommand::Disto { .. } => Some("disto"),
        AnalysisCommand::PoleZero { .. } => Some("pz"),
        AnalysisCommand::Sensitivity { .. } => Some("sens"),
        AnalysisCommand::Tf { .. } => Some("tf"),
        AnalysisCommand::Hb { .. } => Some("hb"),
        AnalysisCommand::MonteCarlo(_) => Some("mc"),
        AnalysisCommand::Pss(_) => Some("pss"),
        AnalysisCommand::Pac(_) => Some("pac"),
        AnalysisCommand::Pnoise(_) => Some("pnoise"),
        AnalysisCommand::Envelope(_) => Some("env"),
        // `.STEP` and `.TEMP` are run axes whose coordinates own the artifact
        // namespace, and `.FOUR` publishes under its own post-process instance
        // identity, so neither owns a physical output namespace.
        AnalysisCommand::Step(_) | AnalysisCommand::Temp { .. } | AnalysisCommand::Four { .. } => {
            None
        }
    }
}

pub(super) fn analysis_output_tag_multiplicities(
    netlist: &Netlist,
) -> std::collections::HashMap<&'static str, usize> {
    let mut counts = std::collections::HashMap::new();
    for tag in netlist.analyses.iter().filter_map(analysis_output_tag) {
        let count = counts.entry(tag).or_insert(0usize);
        *count = count.saturating_add(1);
    }
    counts
}

/// The core analysis family one output tag publishes under.
///
/// Every tag `analysis_output_tag` can return appears here, plus `sparam`,
/// which the command line can request without an authored card. A tag with no
/// family owns no analysis identity and therefore no typed result document.
pub(super) fn output_tag_analysis_kind(tag: &str) -> Option<rspice_core::execution::AnalysisKind> {
    use rspice_core::execution::AnalysisKind;
    match tag {
        "op" => Some(AnalysisKind::Op),
        "dc" => Some(AnalysisKind::Dc),
        "ac" => Some(AnalysisKind::Ac),
        "tran" => Some(AnalysisKind::Tran),
        "noise" => Some(AnalysisKind::Noise),
        // `--sparam` is the command-line spelling of the `.SP` family.
        "sp" | "sparam" => Some(AnalysisKind::Sp),
        "stb" => Some(AnalysisKind::Stb),
        "disto" => Some(AnalysisKind::Distortion),
        "pz" => Some(AnalysisKind::PoleZero),
        "sens" => Some(AnalysisKind::Sensitivity),
        "tf" => Some(AnalysisKind::TransferFunction),
        "hb" => Some(AnalysisKind::HarmonicBalance),
        "mc" => Some(AnalysisKind::MonteCarlo),
        "pss" => Some(AnalysisKind::Pss),
        "pac" => Some(AnalysisKind::Pac),
        "pnoise" => Some(AnalysisKind::PNoise),
        "env" => Some(AnalysisKind::Envelope),
        // The aggregated axis sweep table spans coordinates, so it is not one
        // analysis instance and publishes no typed document of its own.
        _ => None,
    }
}

/// The first `count` canonical identities of one analysis family, minted by
/// the planner exactly as it would mint them for `count` authored cards.
///
/// Every artifact this process publishes for a card the deck authored takes
/// its identity from that deck's own `DeckPlan`. Two callers have no such plan
/// to read and must still name a family instance canonically, and both go
/// through here rather than formatting `sp-001` or `fft-002` by hand:
///
/// - a command-line analysis mode (`--sparam`, `--monte-carlo`) publishes an
///   analysis the deck never authored, so there is no planned card for it. It
///   is single by construction and is therefore planned on its own.
/// - the FFT RAW artifact decoder validates a file this process did not
///   necessarily write, so it has only the artifact's own declared result
///   count to mint the identities it checks against.
///
/// `AnalysisInstanceId` is deliberately not constructible outside
/// `rspice-core`; going through the planner is what keeps the tag spelling,
/// the ordinal base, and the family name decided in exactly one place.
pub(crate) fn canonical_analysis_identities(
    kind: rspice_core::execution::AnalysisKind,
    count: usize,
) -> Result<Vec<AnalysisInstanceId>, DeckPlanError> {
    if count == 0 {
        return Ok(Vec::new());
    }
    let requests = std::iter::repeat_with(|| rspice_core::execution::AnalysisRequest::new(kind))
        .take(count)
        .collect::<Vec<_>>();
    Ok(DeckPlan::new(Vec::new(), requests)?
        .analyses()
        .iter()
        .map(rspice_core::execution::PlannedAnalysis::id)
        .collect())
}

/// Canonical identity of an analysis the command line requested and the deck
/// never authored, such as `--sparam` or `--monte-carlo`.
pub(super) fn command_line_analysis_identity(
    kind: rspice_core::execution::AnalysisKind,
) -> Result<AnalysisInstanceId, DeckPlanError> {
    canonical_analysis_identities(kind, 1)?
        .first()
        .copied()
        .ok_or(DeckPlanError::MissingUpstreamAnalysis {
            card: "command-line analysis mode",
            required: "one planned analysis instance",
        })
}

/// The planned `.FFT` identities of one deck, in authored card order.
pub(super) fn planned_fft_instances(
    post_processes: &[PlannedPostProcess],
) -> impl Iterator<Item = AnalysisInstanceId> + '_ {
    post_processes
        .iter()
        .filter(|post| matches!(post.source(), PostProcessSource::Fft { .. }))
        .map(PlannedPostProcess::id)
}

/// Canonical artifact tags of the deck's authored `.FFT` cards.
///
/// The plan is the only source: an authored `.FFT` card the plan did not name
/// would otherwise publish under a tag this process invented, and the same
/// spectrum would carry two identities across surfaces.
pub(super) fn planned_fft_ids(
    post_processes: &[PlannedPostProcess],
    netlist: &Netlist,
) -> Result<Vec<String>, CliError> {
    let ids = planned_fft_instances(post_processes)
        .map(|id| id.tag())
        .collect::<Vec<_>>();
    if ids.len() != netlist.fft_analyses.len() {
        return Err(CliError::InternalError {
            message: format!(
                "the deck authors {} .FFT card(s) but the canonical plan named {}",
                netlist.fft_analyses.len(),
                ids.len()
            ),
        });
    }
    Ok(ids)
}

pub(super) fn xyce_addresistors_artifact_path(input: &std::path::Path) -> PathBuf {
    let mut name = input.as_os_str().to_os_string();
    name.push("_xyce.cir");
    PathBuf::from(name)
}

pub(super) fn compose_run_label(outer: Option<&str>, inner: Option<&str>) -> Option<String> {
    match (outer, inner) {
        (Some(outer), Some(inner)) => Some(format!("{outer} · {inner}")),
        (Some(label), None) | (None, Some(label)) => Some(label.to_string()),
        (None, None) => None,
    }
}

pub(super) fn conditional_step_schema_path(base: &std::path::Path) -> PathBuf {
    let mut path = tag_output_path(base, "step_schema");
    path.set_extension("json");
    path
}

/// Path of the manifest that names a complete axis coordinate set.
///
/// A deck without a resolved output path publishes no artifacts, so it has no
/// set to describe.
pub(super) fn axis_set_manifest_path(
    args: &RunArgs,
    config: &Config,
    run_label: Option<&str>,
) -> Result<Option<PathBuf>, CliError> {
    let Some(mut base) = resolve_output_path(args.output.clone(), config)? else {
        return Ok(None);
    };
    if let Some(label) = run_label {
        base = tag_output_path(&base, &sanitize_run_tag(label));
    }
    let mut path = tag_output_path(&base, "run_set");
    path.set_extension("json");
    Ok(Some(path))
}

/// `out.csv` + `hot` -> `out.hot.csv` (run-level analog of the
/// per-analysis tagging in `output_path_for`).
/// Where a second document published beside an analysis artifact goes.
///
/// A card whose result is two documents — `.SP DONOISE` publishes the
/// scattering sweep and the port-noise sweep — keeps them under one analysis
/// identity and separates them by file name. Composing the child's name into
/// the parent artifact's own path is what guarantees they differ even for a
/// deck that authors nothing else and therefore publishes under the bare
/// requested path.
pub(super) fn sibling_output_path(path: &std::path::Path, child: &str) -> PathBuf {
    tag_output_path(path, child)
}

pub(super) fn tag_output_path(path: &std::path::Path, tag: &str) -> PathBuf {
    let mut file_name = path
        .file_stem()
        .map(|stem| stem.to_os_string())
        .unwrap_or_default();
    file_name.push(format!(".{tag}"));
    if let Some(ext) = path.extension() {
        file_name.push(".");
        file_name.push(ext);
    }
    path.with_file_name(file_name)
}

/// Reduce a run label to a file-name-safe tag.
pub(super) fn sanitize_run_tag(label: &str) -> String {
    let mut tag: String = label
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    while tag.contains("__") {
        tag = tag.replace("__", "_");
    }
    tag.trim_matches('_').to_string()
}

/// Resolve `-o` against config `output.output_directory`.
///
/// Relative output paths are placed inside the configured directory (created
/// on demand); absolute paths are used as given.
pub(super) fn resolve_output_path(
    output: Option<PathBuf>,
    config: &Config,
) -> Result<Option<PathBuf>, CliError> {
    let Some(path) = output else {
        return Ok(None);
    };
    let Some(dir) = config.output.output_directory.as_ref() else {
        return Ok(Some(path));
    };
    if path.is_absolute() {
        return Ok(Some(path));
    }

    std::fs::create_dir_all(dir).map_err(|e| CliError::OutputError {
        path: dir.clone(),
        source: e,
    })?;
    Ok(Some(dir.join(path)))
}
