//! Xyce `.INITCOND` external-source resolution.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{
    DeviceInitialConditionError, DeviceInitialConditionSource, IncludeProcessor, Netlist,
    NetlistSourceLocation, ParseError, ParseWithAbortError, ensure_parse_not_aborted,
    map_abort_parse_error, parser, poll_parse_abort, poll_parse_text,
};
use crate::abort_signal::{AbortSignal, NoAbort};

/// Maximum accepted size of one external `.INITCOND` data source.
///
/// The format contains only device names and short numeric vectors. A 16 MiB
/// ceiling admits very large extracted designs while preventing accidental or
/// hostile unbounded ingestion in desktop, browser, and mobile frontends.
pub const MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES: usize = 16 * 1024 * 1024;

/// Exact text and canonical identity returned by an `.INITCOND` source
/// provider.
#[derive(Debug, Clone)]
pub struct DeviceInitialConditionSourceText {
    pub resolved_path: PathBuf,
    pub content: Arc<str>,
}

/// Synchronous source-provider boundary for `.INITCOND FILE`.
///
/// File-backed callers use [`IncludeProcessor`]. Sandboxed/WASM callers can
/// implement this trait over an authenticated in-memory source graph without
/// granting the parser filesystem access.
pub trait DeviceInitialConditionSourceProvider {
    fn load_device_initial_condition_source_with_abort(
        &self,
        execution_context: &Path,
        requested_path: &str,
        abort: &dyn AbortSignal,
    ) -> Result<DeviceInitialConditionSourceText, ParseWithAbortError>;
}

impl DeviceInitialConditionSourceProvider for IncludeProcessor {
    fn load_device_initial_condition_source_with_abort(
        &self,
        execution_context: &Path,
        requested_path: &str,
        abort: &dyn AbortSignal,
    ) -> Result<DeviceInitialConditionSourceText, ParseWithAbortError> {
        let resolved_path =
            self.resolve_execution_path_with_abort(execution_context, requested_path, abort)?;
        let content = self.read_source_with_abort(&resolved_path, requested_path, abort)?;
        Ok(DeviceInitialConditionSourceText {
            resolved_path,
            content,
        })
    }
}

impl Netlist {
    /// Resolve `.INITCOND FILE` through a caller-supplied source provider.
    pub fn resolve_device_initial_condition_source(
        &mut self,
        provider: &dyn DeviceInitialConditionSourceProvider,
    ) -> Result<(), ParseError> {
        match self.resolve_device_initial_condition_source_with_abort(provider, &NoAbort) {
            Ok(()) => Ok(()),
            Err(ParseWithAbortError::Parse(error)) => Err(error),
            Err(ParseWithAbortError::Aborted) => {
                unreachable!("NoAbort cannot cancel INITCOND source resolution")
            }
        }
    }

    /// Resolve `.INITCOND FILE` transactionally with cooperative cancellation.
    pub fn resolve_device_initial_condition_source_with_abort(
        &mut self,
        provider: &dyn DeviceInitialConditionSourceProvider,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        ensure_parse_not_aborted(abort)?;
        let Some(directive) = self.device_initial_conditions.as_ref() else {
            return Ok(());
        };
        let DeviceInitialConditionSource::File {
            requested_path,
            resolved_path,
            ..
        } = &directive.source
        else {
            return Ok(());
        };
        if resolved_path.is_some() {
            return Ok(());
        }

        let directive_origin = directive.origin.clone();
        let requested_path = requested_path.clone();
        let execution_context = self
            .source_path
            .as_deref()
            .or(directive_origin.path.as_deref())
            .unwrap_or_else(|| Path::new("."));
        let loaded = provider
            .load_device_initial_condition_source_with_abort(
                execution_context,
                &requested_path,
                abort,
            )
            .map_err(|error| {
                map_abort_parse_error(error, |_| {
                    ParseError::DeviceInitialCondition(Box::new(
                        DeviceInitialConditionError::SourceUnavailable {
                            origin: directive_origin.clone(),
                            requested_path: requested_path.clone(),
                        },
                    ))
                })
            })?;

        if loaded.content.len() > MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedSource {
                    origin: directive_origin,
                    requested_path,
                    record_origin: NetlistSourceLocation::in_file(&loaded.resolved_path, 1),
                    detail: format!(
                        "source is {} bytes; maximum is {}",
                        loaded.content.len(),
                        MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES
                    ),
                },
            ))
            .into());
        }

        let mut entries = Vec::new();
        for (line_index, raw_line) in loaded.content.lines().enumerate() {
            poll_parse_abort(abort, line_index)?;
            poll_parse_text(abort, raw_line)?;
            let line_number = line_index + 1;
            let record =
                super::parser::strip_device_initial_condition_record_comment(raw_line).trim();
            if record.is_empty() || record.starts_with('*') {
                continue;
            }
            let record_origin = NetlistSourceLocation::in_file(&loaded.resolved_path, line_number);
            let parsed = parser::parse_device_initial_condition_record(
                record,
                line_number,
                &self.params,
                &record_origin,
            )
            .map_err(|error| match error {
                ParseError::DeviceInitialCondition(inner)
                    if matches!(*inner, DeviceInitialConditionError::NonFiniteValue { .. }) =>
                {
                    ParseError::DeviceInitialCondition(inner)
                }
                other => ParseError::DeviceInitialCondition(Box::new(
                    DeviceInitialConditionError::MalformedSource {
                        origin: directive_origin.clone(),
                        requested_path: requested_path.clone(),
                        record_origin,
                        detail: other.to_string(),
                    },
                )),
            })?;
            entries.extend(parsed);
        }
        ensure_parse_not_aborted(abort)?;

        if entries.is_empty() {
            return Err(ParseError::DeviceInitialCondition(Box::new(
                DeviceInitialConditionError::MalformedSource {
                    origin: directive_origin,
                    requested_path,
                    record_origin: NetlistSourceLocation::in_file(&loaded.resolved_path, 1),
                    detail: "source contains no device initial-condition records".to_string(),
                },
            ))
            .into());
        }

        ensure_parse_not_aborted(abort)?;
        let directive = self
            .device_initial_conditions
            .as_mut()
            .expect("directive remains present during transactional resolution");
        directive.entries = entries;
        directive.source = DeviceInitialConditionSource::File {
            requested_path,
            resolved_path: Some(loaded.resolved_path),
            content_identity: Some(blake3::hash(loaded.content.as_bytes()).to_hex().to_string()),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::{
        DeviceInitialConditionSource, ElementKind, SealedSourceBundle, SealedSourceEdge,
        flatten_netlist,
    };
    use std::sync::atomic::{AtomicU64, Ordering};

    fn deck_path(name: &str) -> PathBuf {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        std::env::temp_dir()
            .join(format!(
                "rspice-initcond-{}-{}-{name}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ))
            .join("deck.cir")
    }

    fn capacitor_initial_voltage(netlist: &Netlist, name: &str) -> Option<f64> {
        let flattened = flatten_netlist(netlist).expect("INITCOND deck flattens");
        flattened
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .and_then(|element| match &element.kind {
                ElementKind::Capacitor {
                    initial_voltage, ..
                } => *initial_voltage,
                _ => None,
            })
    }

    #[test]
    fn inline_directive_retains_typed_entries_and_source_provenance() {
        let netlist = Netlist::parse(
            "inline initcond\n\
             .INITCOND C1 IC=4 XTOP:MN1 IC=2, 0\n\
             C1 1 0 1u\n\
             .END\n",
        )
        .expect("inline INITCOND parses");
        let directive = netlist
            .device_initial_conditions
            .as_ref()
            .expect("typed INITCOND directive retained");
        assert_eq!(directive.source, DeviceInitialConditionSource::Inline);
        assert_eq!(directive.origin, NetlistSourceLocation::in_memory(2));
        assert_eq!(directive.entries.len(), 2);
        assert_eq!(directive.entries[0].device, "C1");
        assert_eq!(directive.entries[0].values, vec![4.0]);
        assert_eq!(directive.entries[1].device, "XTOP:MN1");
        assert_eq!(directive.entries[1].values, vec![2.0, 0.0]);
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unsupported-dot-command")
        );
    }

    #[test]
    fn duplicate_card_is_rejected_before_malformed_second_card_is_parsed() {
        let error = Netlist::parse(
            "duplicate initcond\n\
             .INITCOND C1 IC=1\n\
             .INITCOND this second card is deliberately malformed\n\
             C1 1 0 1u\n\
             .END\n",
        )
        .expect_err("second INITCOND card must fail");
        assert!(matches!(
            error,
            ParseError::DeviceInitialCondition(inner)
                if matches!(*inner, DeviceInitialConditionError::DuplicateDirective { .. })
        ));
    }

    #[test]
    fn duplicate_card_across_include_boundary_retains_both_source_locations() {
        let deck = deck_path("duplicate-include");
        let dir = deck.parent().expect("deck has parent");
        let child = dir.join("child.inc");
        std::fs::create_dir_all(dir).expect("create include fixture");
        std::fs::write(&child, ".INITCOND C1 IC=1\n").expect("write child directive");
        let source = "included duplicate\n\
                      .INCLUDE child.inc\n\
                      .INITCOND malformed second card\n\
                      C1 1 0 1u\n\
                      .END\n";
        std::fs::write(&deck, source).expect("write root deck");

        let error = Netlist::parse_with_path(source, &deck)
            .expect_err("duplicate across include boundary must fail");
        let ParseError::DeviceInitialCondition(inner) = error else {
            panic!("expected typed INITCOND duplicate error");
        };
        let DeviceInitialConditionError::DuplicateDirective { first, duplicate } = *inner else {
            panic!("expected duplicate directive payload");
        };
        let canonical_child = child.canonicalize().expect("child path canonicalizes");
        assert_eq!(first.path.as_deref(), Some(canonical_child.as_path()));
        assert_eq!(first.line, 1);
        assert_eq!(duplicate.path.as_deref(), Some(deck.as_path()));
        assert_eq!(duplicate.line, 3);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn inline_overlay_is_last_wins_and_preserves_empty_hierarchy_components() {
        let last_wins = Netlist::parse(
            "last wins\n\
             .INITCOND C1 IC=1 C1 IC=2\n\
             C1 1 0 1u IC=9\n\
             .END\n",
        )
        .expect("duplicate target entries parse");
        assert_eq!(capacitor_initial_voltage(&last_wins, "C1"), Some(2.0));

        let malformed_hierarchy = Netlist::parse(
            "empty hierarchy component\n\
             .SUBCKT CELL p n\n\
             C1 p n 1u\n\
             .ENDS\n\
             .INITCOND X1::C1 IC=7\n\
             X1 1 0 CELL\n\
             .END\n",
        )
        .expect("unmatched target spelling remains legal");
        assert_eq!(
            capacitor_initial_voltage(&malformed_hierarchy, "X1.C1"),
            None,
            "X1::C1 must not collapse onto X1.C1"
        );
    }

    #[test]
    fn hierarchy_overlay_overrides_authored_mos_ic_without_touching_l_or_unknown_targets() {
        let netlist = Netlist::parse(
            "hierarchical initcond\n\
             .SUBCKT CELL d g s b\n\
             M1 d g s b MOD IC=20000,10000\n\
             L1 d s 1m IC=10\n\
             .ENDS\n\
             .INITCOND X1:M1 IC=2,0 X1:L1 IC=99 X1 IC=8 NO_SUCH_DEVICE IC=7\n\
             X1 1 2 0 0 CELL\n\
             .MODEL MOD NMOS\n\
             .END\n",
        )
        .expect("hierarchical INITCOND parses");
        let flattened = flatten_netlist(&netlist).expect("ignored targets do not fail flattening");
        let mos = flattened
            .iter()
            .find(|element| element.name == "X1.M1")
            .expect("nested MOS exists");
        let ElementKind::Mosfet {
            instance_params, ..
        } = &mos.kind
        else {
            panic!("X1.M1 must remain a MOSFET");
        };
        assert_eq!(
            instance_params
                .iter()
                .filter(|(name, _)| name.starts_with("IC_"))
                .cloned()
                .collect::<Vec<_>>(),
            vec![("IC_VDS".to_string(), 2.0), ("IC_VGS".to_string(), 0.0)]
        );

        let inductor = flattened
            .iter()
            .find(|element| element.name == "X1.L1")
            .expect("nested inductor exists");
        assert!(matches!(
            inductor.kind,
            ElementKind::Inductor {
                initial_current: Some(value),
                ..
            } if value == 10.0
        ));
    }

    #[test]
    fn eligible_device_vector_arity_is_validated_at_elaboration() {
        let netlist = Netlist::parse(
            "bad capacitor vector\n\
             .INITCOND C1 IC=1,2\n\
             C1 1 0 1u\n\
             .END\n",
        )
        .expect("syntax is valid before device arity is known");
        let error = flatten_netlist(&netlist).expect_err("capacitor vector arity must fail");
        assert!(matches!(
            error,
            ParseError::DeviceInitialCondition(inner)
                if matches!(*inner, DeviceInitialConditionError::InvalidArity { actual: 2, .. })
        ));
    }

    #[test]
    fn matched_ordinary_device_without_real_ic_support_fails_closed() {
        let netlist = Netlist::parse(
            "unsupported diode initcond\n\
             .INITCOND D1 IC=.7\n\
             D1 1 0 DMOD\n\
             .MODEL DMOD D\n\
             .END\n",
        )
        .expect("INITCOND syntax parses before target type is elaborated");
        let error = flatten_netlist(&netlist).expect_err("unsupported matched target must fail");
        assert!(matches!(
            error,
            ParseError::DeviceInitialCondition(inner)
                if matches!(
                    *inner,
                    DeviceInitialConditionError::UnsupportedTarget {
                        ref device,
                        ref device_type,
                        ..
                    } if device == "D1" && device_type == "diode"
                )
        ));
    }

    #[test]
    fn ordinary_path_parse_uses_process_cwd_not_deck_directory() {
        let deck = deck_path("process-cwd");
        let authored_dir = deck.parent().expect("deck has a parent");
        std::fs::create_dir_all(authored_dir).expect("create deck fixture");
        let unique_name = format!(
            "{}.initcond.dat",
            authored_dir
                .file_name()
                .expect("fixture directory has name")
                .to_string_lossy()
        );
        let cwd_source = std::env::current_dir()
            .expect("current directory is available")
            .join(&unique_name);
        std::fs::write(authored_dir.join(&unique_name), "C1 IC=3\n")
            .expect("write deck-directory decoy");
        std::fs::write(&cwd_source, "C1 IC=8\n").expect("write process-CWD source");
        let source = format!("cwd initcond\n.INITCOND FILE \"{unique_name}\"\nC1 1 0 1u\n.END\n");
        std::fs::write(&deck, &source).expect("write deck fixture");

        let netlist = Netlist::parse_with_path(&source, &deck)
            .expect("ordinary path parse resolves CWD file");
        assert_eq!(capacitor_initial_voltage(&netlist, "C1"), Some(8.0));

        let _ = std::fs::remove_file(cwd_source);
        let _ = std::fs::remove_dir_all(authored_dir);
    }

    #[test]
    fn file_source_uses_execution_directory_not_authored_source_directory() {
        let deck = deck_path("execution-directory");
        let authored_dir = deck.parent().expect("deck has a parent");
        let execution_dir = authored_dir.join("execution");
        std::fs::create_dir_all(&execution_dir).expect("create execution fixture");
        std::fs::write(authored_dir.join("init cond.dat"), "C1 IC=3\n")
            .expect("write authored-directory decoy");
        std::fs::write(execution_dir.join("init cond.dat"), "C1 IC=9\n")
            .expect("write execution-directory source");
        let source = "file initcond\n.INITCOND FILE \"init cond.dat\"\nC1 1 0 1u\n.END\n";
        std::fs::write(&deck, source).expect("write deck fixture");

        let netlist = Netlist::parse_with_path_and_execution_dir(
            source,
            &deck,
            &execution_dir,
            super::super::NetlistParseOptions::default(),
        )
        .expect("execution-directory INITCOND source resolves");
        assert_eq!(capacitor_initial_voltage(&netlist, "C1"), Some(9.0));
        let directive = netlist
            .device_initial_conditions
            .expect("directive retained");
        assert!(matches!(
            directive.source,
            DeviceInitialConditionSource::File {
                resolved_path: Some(path),
                content_identity: Some(_),
                ..
            } if path == execution_dir.join("init cond.dat")
        ));

        let _ = std::fs::remove_dir_all(authored_dir);
    }

    #[test]
    fn sealed_source_provider_resolves_authenticated_execution_edge() {
        let root = deck_path("sealed");
        let data = root
            .parent()
            .expect("root has parent")
            .join("sealed-initcond.dat");
        let source = "sealed initcond\n.INITCOND FILE initcond.dat\nC1 1 0 1u\n.END\n";
        let bundle = SealedSourceBundle::try_new_with_edges(
            [
                (root.clone(), source.to_string()),
                (data.clone(), "C1 IC=6\n".to_string()),
            ],
            [SealedSourceEdge {
                owner: root.clone(),
                requested_path: "initcond.dat".to_string(),
                target: data.clone(),
            }],
        )
        .expect("sealed source graph is valid");
        let provider = IncludeProcessor::new_sealed(&root, bundle);
        let mut netlist = Netlist::parse(source).expect("unresolved file directive parses");
        netlist.source_path = Some(root);
        netlist
            .resolve_device_initial_condition_source(&provider)
            .expect("sealed edge resolves INITCOND source");
        assert_eq!(capacitor_initial_voltage(&netlist, "C1"), Some(6.0));
        assert!(matches!(
            netlist
                .device_initial_conditions
                .as_ref()
                .expect("directive retained")
                .source,
            DeviceInitialConditionSource::File {
                resolved_path: Some(ref path),
                content_identity: Some(_),
                ..
            } if path == &data
        ));
    }

    #[test]
    fn whitespace_only_file_is_malformed_and_resolution_is_transactional() {
        #[derive(Debug)]
        struct FixedProvider;
        impl DeviceInitialConditionSourceProvider for FixedProvider {
            fn load_device_initial_condition_source_with_abort(
                &self,
                _execution_context: &Path,
                _requested_path: &str,
                _abort: &dyn AbortSignal,
            ) -> Result<DeviceInitialConditionSourceText, ParseWithAbortError> {
                Ok(DeviceInitialConditionSourceText {
                    resolved_path: PathBuf::from("noinits.dat"),
                    content: Arc::from("   \r\n\t \r\n"),
                })
            }
        }

        let mut netlist =
            Netlist::parse("empty file\n.INITCOND FILE noinits.dat\nC1 1 0 1u\n.END\n")
                .expect("unresolved file directive parses");
        let error = netlist
            .resolve_device_initial_condition_source(&FixedProvider)
            .expect_err("whitespace-only INITCOND source must fail");
        assert!(matches!(
            error,
            ParseError::DeviceInitialCondition(inner)
                if matches!(*inner, DeviceInitialConditionError::MalformedSource { .. })
        ));
        let directive = netlist
            .device_initial_conditions
            .expect("failed resolution retains directive");
        assert!(directive.entries.is_empty());
        assert!(matches!(
            directive.source,
            DeviceInitialConditionSource::File {
                resolved_path: None,
                content_identity: None,
                ..
            }
        ));
    }

    #[test]
    fn external_resolution_discards_valid_prefix_when_a_later_record_is_malformed() {
        #[derive(Debug)]
        struct PartiallyMalformedProvider;
        impl DeviceInitialConditionSourceProvider for PartiallyMalformedProvider {
            fn load_device_initial_condition_source_with_abort(
                &self,
                _execution_context: &Path,
                _requested_path: &str,
                _abort: &dyn AbortSignal,
            ) -> Result<DeviceInitialConditionSourceText, ParseWithAbortError> {
                Ok(DeviceInitialConditionSourceText {
                    resolved_path: PathBuf::from("partially-malformed.dat"),
                    content: Arc::from("C1 IC=5\nthis-record-is-malformed\n"),
                })
            }
        }

        let mut netlist =
            Netlist::parse("transactional file\n.INITCOND FILE partial.dat\nC1 1 0 1u\n.END\n")
                .expect("unresolved file directive parses");
        let error = netlist
            .resolve_device_initial_condition_source(&PartiallyMalformedProvider)
            .expect_err("malformed suffix must reject the whole source");
        assert!(matches!(
            error,
            ParseError::DeviceInitialCondition(inner)
                if matches!(*inner, DeviceInitialConditionError::MalformedSource { .. })
        ));
        let directive = netlist
            .device_initial_conditions
            .expect("failed resolution retains directive");
        assert!(
            directive.entries.is_empty(),
            "valid prefix must not leak into the AST after a later failure"
        );
    }

    #[test]
    fn cancellation_at_commit_boundary_leaves_external_directive_unresolved() {
        #[derive(Debug)]
        struct OneRecordProvider;
        impl DeviceInitialConditionSourceProvider for OneRecordProvider {
            fn load_device_initial_condition_source_with_abort(
                &self,
                _execution_context: &Path,
                _requested_path: &str,
                _abort: &dyn AbortSignal,
            ) -> Result<DeviceInitialConditionSourceText, ParseWithAbortError> {
                Ok(DeviceInitialConditionSourceText {
                    resolved_path: PathBuf::from("commit-boundary.dat"),
                    content: Arc::from("C1 IC=5\n"),
                })
            }
        }

        let mut netlist =
            Netlist::parse("cancel file\n.INITCOND FILE cancel.dat\nC1 1 0 1u\n.END\n")
                .expect("unresolved file directive parses");
        let abort = crate::abort_signal::CountingAbort::new(4);
        let result =
            netlist.resolve_device_initial_condition_source_with_abort(&OneRecordProvider, &abort);
        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert_eq!(
            abort.count(),
            5,
            "edge-triggered abort must fire at the final pre-commit poll"
        );
        let directive = netlist
            .device_initial_conditions
            .expect("cancelled resolution retains directive");
        assert!(directive.entries.is_empty());
        assert!(matches!(
            directive.source,
            DeviceInitialConditionSource::File {
                resolved_path: None,
                content_identity: None,
                ..
            }
        ));
    }

    #[test]
    fn nonfinite_inline_values_have_a_typed_failure() {
        let error = Netlist::parse("nonfinite initcond\n.INITCOND C1 IC=1e309\nC1 1 0 1u\n.END\n")
            .expect_err("nonfinite INITCOND value must fail");
        assert!(matches!(
            error,
            ParseError::DeviceInitialCondition(inner)
                if matches!(*inner, DeviceInitialConditionError::NonFiniteValue { .. })
        ));
    }

    fn bug702_path(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/xyce/Netlists/Certification_Tests/BUG_702")
            .join(name)
    }

    fn parse_bug702_with_family_execution_dir(name: &str) -> Result<Netlist, ParseError> {
        let path = bug702_path(name);
        let source = Netlist::read_source(&path)?;
        Netlist::parse_with_path_and_execution_dir(
            &source,
            &path,
            path.parent().expect("BUG702 deck has family directory"),
            super::super::NetlistParseOptions::default(),
        )
    }

    #[test]
    fn bug702_error_decks_expose_exact_core_failure_classes() {
        for name in ["dup-external.cir", "dup-inlined.cir"] {
            let error = parse_bug702_with_family_execution_dir(name)
                .expect_err("duplicate INITCOND oracle deck must fail");
            assert!(
                matches!(
                    &error,
                    ParseError::DeviceInitialCondition(inner)
                        if matches!(
                            **inner,
                            DeviceInitialConditionError::DuplicateDirective { .. }
                        )
                ),
                "{name}: unexpected error {error}"
            );
        }

        let error = parse_bug702_with_family_execution_dir("missing-initcond.cir")
            .expect_err("missing INITCOND source must fail");
        assert!(
            matches!(
                &error,
                ParseError::DeviceInitialCondition(inner)
                    if matches!(
                        **inner,
                        DeviceInitialConditionError::SourceUnavailable {
                            ref requested_path,
                            ..
                        } if requested_path == "ic.dat"
                    )
            ),
            "unexpected missing-file error {error}"
        );

        let error = parse_bug702_with_family_execution_dir("empty-initcond.cir")
            .expect_err("whitespace-only INITCOND source must fail");
        assert!(
            matches!(
                &error,
                ParseError::DeviceInitialCondition(inner)
                    if matches!(
                        **inner,
                        DeviceInitialConditionError::MalformedSource {
                            ref requested_path,
                            ..
                        } if requested_path == "noinits.dat"
                    )
            ),
            "unexpected malformed-file error {error}"
        );
    }

    #[test]
    fn bug702_inline_multiple_and_precedence_overlays_match_oracle_semantics() {
        let multiple = parse_bug702_with_family_execution_dir("inlined-multiple.cir")
            .expect("BUG702 multi-device inline deck parses");
        let flattened = flatten_netlist(&multiple).expect("BUG702 multi-device deck flattens");
        assert!(matches!(
            flattened
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("C1"))
                .map(|element| &element.kind),
            Some(ElementKind::Capacitor {
                initial_voltage: Some(value),
                ..
            }) if *value == 400.0
        ));
        assert!(matches!(
            flattened
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("XNLR1.CABS"))
                .map(|element| &element.kind),
            Some(ElementKind::Capacitor {
                initial_voltage: Some(value),
                ..
            }) if *value == 0.0
        ));
        assert!(matches!(
            flattened
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case("L1"))
                .map(|element| &element.kind),
            Some(ElementKind::Inductor {
                initial_current: Some(value),
                ..
            }) if *value == 10.0
        ));

        let precedence = parse_bug702_with_family_execution_dir("precedence.cir")
            .expect("BUG702 precedence deck parses");
        let flattened = flatten_netlist(&precedence).expect("BUG702 precedence deck flattens");
        let mos = flattened
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("XINV1.MN1"))
            .expect("precedence target MOS exists");
        let ElementKind::Mosfet {
            instance_params, ..
        } = &mos.kind
        else {
            panic!("precedence target must be MOS");
        };
        assert_eq!(
            instance_params
                .iter()
                .filter(|(name, _)| name.starts_with("IC_"))
                .cloned()
                .collect::<Vec<_>>(),
            vec![("IC_VDS".to_string(), 2.0), ("IC_VGS".to_string(), 0.0)]
        );
    }

    #[test]
    fn bug702_external_source_resolves_exact_mos_entry() {
        let external = parse_bug702_with_family_execution_dir("external.cir")
            .expect("BUG702 external INITCOND deck parses");
        let directive = external
            .device_initial_conditions
            .as_ref()
            .expect("external directive retained");
        assert_eq!(directive.entries.len(), 1);
        assert_eq!(directive.entries[0].device, "XiNv1:mn1");
        assert_eq!(directive.entries[0].values, vec![2.0, 0.0]);
        let flattened = flatten_netlist(&external).expect("external BUG702 deck flattens");
        let mos = flattened
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("XINV1.MN1"))
            .expect("external target MOS exists");
        let ElementKind::Mosfet {
            instance_params, ..
        } = &mos.kind
        else {
            panic!("external target must be MOS");
        };
        assert_eq!(
            instance_params
                .iter()
                .filter(|(name, _)| name.starts_with("IC_"))
                .cloned()
                .collect::<Vec<_>>(),
            vec![("IC_VDS".to_string(), 2.0), ("IC_VGS".to_string(), 0.0)]
        );
    }
}
