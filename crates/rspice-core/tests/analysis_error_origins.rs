//! Typed card failures retain their physical owner through delayed binding.
use rspice_core::netlist::{
    AnalysisCard, AnalysisCardError, AnalysisCardIssue, NetlistSourceLocation, ParseError,
    ParseWithAbortError, SealedSourceBundle, SealedSourceEdge,
};
use rspice_core::{Netlist, NoAbort};

fn root_path() -> std::path::PathBuf {
    if cfg!(windows) {
        "C:/rspice-analysis-errors/root.cir".into()
    } else {
        "/rspice-analysis-errors/root.cir".into()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn constructor_keeps_unattributed_line_and_typed_issue() {
    let issue = AnalysisCardIssue::MissingField { field: "FUND" };
    let error = AnalysisCardError::new(AnalysisCard::Pss, 9, issue.clone());
    assert_eq!(error.source_location(), NetlistSourceLocation::in_memory(9));
    assert_eq!(error.issue, issue);
    assert_eq!(
        error.to_string(),
        ".PSS at line 9: missing required field FUND"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn in_memory_cards_keep_line_only_public_locations() {
    let error = Netlist::parse("Analysis origin\n.PSS FUND=1G HARMS=0\n.end\n").unwrap_err();
    assert_eq!(
        error.source_location(),
        Some(NetlistSourceLocation::in_memory(2))
    );
    assert!(error.to_string().starts_with(".PSS at line 2:"));
    let ParseError::AnalysisCard(error) = error else {
        panic!("typed card")
    };
    assert_eq!(error.origin, Some(NetlistSourceLocation::in_memory(2)));
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn root_and_scoped_errors_keep_physical_lines_and_structured_issues() {
    for (opening, closing, line) in [("", "", 3), (".subckt child p\n", ".ends\n", 4)] {
        let source = format!(
            "Card origin\n{opening}.OP\n.PSS FUND=1G HARMS={{count}}\n.param count=0\n{closing}.end\n"
        );
        let root = root_path();
        let sources =
            SealedSourceBundle::try_new_with_edges([(root.clone(), source.clone())], []).unwrap();
        let error = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
            &source,
            &root,
            sources,
            Default::default(),
            &NoAbort,
        )
        .unwrap_err();
        let ParseWithAbortError::Parse(error) = error else {
            panic!("parse failure")
        };
        let expected = NetlistSourceLocation::in_file(root_path(), line);
        assert_eq!(error.source_location(), Some(expected.clone()));
        let ParseError::AnalysisCard(error) = error else {
            panic!("typed card failure")
        };
        assert_eq!(error.card, AnalysisCard::Pss);
        assert_eq!(error.line, line);
        assert_eq!(error.origin, Some(expected));
        assert!(matches!(
            error.issue,
            AnalysisCardIssue::InvalidNumber {
                field: "HARMS",
                value: 0.0,
                ..
            }
        ));
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn nested_sealed_includes_and_continuations_preserve_typed_card_ownership() {
    let root = root_path();
    let parent = root.with_file_name("parent.inc");
    let child = root.with_file_name("child.inc");
    let source = "Card origins\n.OP\n.include parent.inc\n.param count=0\n.end\n";
    for scoped in [false, true] {
        for (card, expected_issue) in [
            (".PSS FUND=1G\n+ HARMS={count}\n", "number"),
            (".PSS FUND=1G EXTRA=1\n", "keyword"),
            (".PSS FUND=1G FUND=2G\n", "duplicate"),
            (".PSS FUND={missing}\n", "missing"),
        ] {
            let line = if scoped { 3 } else { 2 };
            let body = if scoped {
                format!("* included\n.subckt cell p\n{card}.ends\n")
            } else {
                format!("* included\n{card}")
            };
            let bundle = SealedSourceBundle::try_new_with_edges(
                [
                    (root.clone(), source.to_owned()),
                    (parent.clone(), "* parent\n.include child.inc\n".to_owned()),
                    (child.clone(), body),
                ],
                [
                    SealedSourceEdge {
                        owner: root.clone(),
                        requested_path: "parent.inc".into(),
                        target: parent.clone(),
                    },
                    SealedSourceEdge {
                        owner: parent.clone(),
                        requested_path: "child.inc".into(),
                        target: child.clone(),
                    },
                ],
            )
            .unwrap();
            let error = Netlist::parse_with_path_and_sealed_sources_and_options_and_abort(
                source,
                &root,
                bundle,
                Default::default(),
                &NoAbort,
            )
            .unwrap_err();
            let ParseWithAbortError::Parse(error) = error else {
                panic!("parse failure")
            };
            assert_eq!(
                error.source_location(),
                Some(NetlistSourceLocation::in_file(&child, line))
            );
            assert!(
                error.to_string().contains(&format!("child.inc:{line}")),
                "{error}"
            );
            assert!(!error.to_string().contains("root.cir"), "{error}");
            let ParseError::AnalysisCard(error) = error else {
                panic!("typed card failure")
            };
            assert_eq!(error.card, AnalysisCard::Pss);
            assert_eq!(error.line, line);
            match expected_issue {
                "number" => assert!(matches!(
                    error.issue,
                    AnalysisCardIssue::InvalidNumber { field: "HARMS", .. }
                )),
                "keyword" => assert!(
                    matches!(error.issue, AnalysisCardIssue::UnknownKeyword { ref keyword } if keyword == "EXTRA")
                ),
                "duplicate" => assert!(matches!(
                    error.issue,
                    AnalysisCardIssue::DuplicateKeyword { keyword: "FUND" }
                )),
                "missing" => assert!(matches!(
                    error.issue,
                    AnalysisCardIssue::MissingField { field: "FUND" }
                )),
                _ => unreachable!(),
            }
        }
    }
}
