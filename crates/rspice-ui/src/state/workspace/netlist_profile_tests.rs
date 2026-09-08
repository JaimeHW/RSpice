//! Profile admission and adaptation must describe the source the parser executes.

use super::NetlistExecutionProfile;

#[test]
fn profile_validation_checks_deferred_file_waveforms_before_admission() {
    let profile = NetlistExecutionProfile::PspiceDeclarativeV2;
    for (waveform, file_backed) in [
        ("DC {level} PWL(0 0 1 {level})", false),
        ("DC {level} PWL\n+ FILE=\"unsealed.csv\"", true),
    ] {
        let source = format!(
            "deck\n.probe V(out)\n.subckt source out params: level=1\nV1 out 0 {waveform}\n.ends\nX1 out source level=2\nR1 out 0 1k\n.op\n.end\n"
        );
        let adapted = profile.adapt_source(&source).unwrap();
        let parsed = rspice_core::Netlist::parse(&adapted).unwrap();
        let result = profile.validate_parsed_netlist(&parsed);
        if file_backed {
            let error = result.expect_err("resolved file inputs cannot be admitted");
            assert!(error.contains("unsealed file-backed PWL"), "{error}");
        } else {
            result.expect("scope-resolved inline samples remain valid");
        }
    }
}

#[test]
fn inline_pwl_sources_may_use_file_as_a_node_or_parameter_name() {
    for source in [
        "deck\n.probe V(file)\nV1 file 0 PWL(0 0 1 1)\nR1 file 0 1k\n.op\n.end\n",
        "deck\n.probe V(out)\n.param file=1\nV1 out 0 PWL(0 0 1 {file})\nR1 out 0 1k\n.op\n.end\n",
    ] {
        let profile = NetlistExecutionProfile::PspiceDeclarativeV2;
        let adapted = profile
            .adapt_source(source)
            .expect("inline samples do not name a dependency");
        let parsed = rspice_core::Netlist::parse(&adapted).unwrap();
        profile.validate_parsed_netlist(&parsed).unwrap();
    }
}

#[test]
fn export_headers_and_quoted_semicolons_survive_adaptation() {
    for (profile, header) in [
        (
            NetlistExecutionProfile::SpectreSpiceV1,
            "simulator lang=spice; export",
        ),
        (
            NetlistExecutionProfile::AdsSpiceExportV1,
            "Options ResourceUsage=yes UseNutmegFormat=yes TopDesignName=\"semi;colon\"; export",
        ),
    ] {
        for title_header in [true, false] {
            let source = format!(
                "{}{header}\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n",
                if title_header { "" } else { "deck\n" }
            );
            let adapted = profile.adapt_source(&source).unwrap();
            profile.validate_executable_source(&adapted).unwrap();
            let parsed = rspice_core::Netlist::parse(&adapted).unwrap();
            assert_eq!(parsed.elements.len(), 2);
            if title_header {
                assert_eq!(parsed.title, header);
            }
        }
    }
    assert!(
        NetlistExecutionProfile::HspiceDeclarativeV1
            .adapt_source("deck\n.optionpost=2\n.op\n.end\n")
            .is_err()
    );
}

#[test]
fn pwl_mentions_in_titles_and_comments_do_not_create_external_dependencies() {
    let source = "PWL FILE notes\n.probe V(out)\nV1 out 0 PWL(0 0 1 1); FILE reference\nR1 out 0 1k\n.op\n.end\n";
    NetlistExecutionProfile::PspiceDeclarativeV2
        .adapt_source(source)
        .unwrap();
    let file_source = "deck\n.probe V(out)\nV1 out 0 PWL\n+ FILE \"external.txt\"\n.op\n.end\n";
    let profile = NetlistExecutionProfile::PspiceDeclarativeV2;
    let adapted = profile.adapt_source(file_source).unwrap();
    assert!(
        profile
            .validate_parsed_netlist(&rspice_core::Netlist::parse(&adapted).unwrap())
            .is_err()
    );
}

fn profile_markers() -> [(NetlistExecutionProfile, &'static str); 5] {
    [
        (
            NetlistExecutionProfile::HspiceDeclarativeV1,
            ".option post=2",
        ),
        (
            NetlistExecutionProfile::PspiceDeclarativeV1,
            ".probe V(out)",
        ),
        (
            NetlistExecutionProfile::PspiceDeclarativeV2,
            ".probe64 V(out)",
        ),
        (
            NetlistExecutionProfile::SpectreSpiceV1,
            "simulator lang=spice",
        ),
        (
            NetlistExecutionProfile::AdsSpiceExportV1,
            "Options ResourceUsage=yes UseNutmegFormat=yes TopDesignName=\"test\"",
        ),
    ]
}

#[test]
fn profile_validation_preserves_end_titles_and_commented_markers() {
    for (profile, marker) in profile_markers() {
        for title in [".end", "ordinary title"] {
            let source = format!(
                "{title}\r\n{marker}; author note\r\nV1 out 0 1\r\nR1 out 0 1k\r\n.op\r\n.end; done\r\n"
            );
            let adapted = profile
                .adapt_source(&source)
                .expect("SPICE titles and comments are not commands");
            profile
                .validate_executable_source(&adapted)
                .expect("adapter output satisfies its profile");
            let parsed = rspice_core::Netlist::parse(&adapted).expect("adapted deck parses");
            assert_eq!(parsed.title, title, "{profile:?}: {adapted}");
            assert_eq!(parsed.elements.len(), 2, "{adapted}");
            assert_eq!(parsed.analyses.len(), 1, "{adapted}");
            assert_eq!(adapted.lines().count(), source.lines().count());
            assert!(adapted.starts_with(&format!("{title}\r\n")), "{adapted:?}");
            assert!(adapted.ends_with(".end; done\r\n"), "{adapted:?}");
        }
    }
}

#[test]
fn profile_markers_after_commented_termination_do_not_qualify_a_deck() {
    for (profile, marker) in profile_markers() {
        let source =
            format!("title\nV1 out 0 1\nR1 out 0 1k\n.op\n.end; actual termination\n{marker}\n");
        let error = profile
            .validate_source(&source)
            .expect_err("ignored tail text is not an executable profile marker");
        assert!(error.contains("after .END"), "{profile:?}: {error}");
    }
}

#[test]
fn command_titles_do_not_supply_required_body_markers() {
    for (profile, marker) in profile_markers().into_iter().take(3) {
        let source = format!("{marker}\nV1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
        assert!(profile.validate_source(&source).is_err(), "{profile:?}");
    }
}

#[test]
fn command_like_titles_do_not_restrict_profile_admission() {
    for (profile, title, marker) in [
        (NetlistExecutionProfile::Spice3NgspiceV1, ".control", ""),
        (NetlistExecutionProfile::Spice3NgspiceV2, ".control", ""),
        (
            NetlistExecutionProfile::PspiceDeclarativeV2,
            ".stimulus",
            ".probe V(out)\n",
        ),
        (
            NetlistExecutionProfile::HspiceDeclarativeV1,
            ".alter",
            ".option post=2\n",
        ),
    ] {
        let source = format!("{title}\n{marker}V1 out 0 1\nR1 out 0 1k\n.op\n.end\n");
        let adapted = profile
            .adapt_source(&source)
            .expect("the first physical record is a title");
        let parsed =
            rspice_core::Netlist::parse(&adapted).expect("title was not rewritten as a command");
        assert_eq!(parsed.title, title, "{profile:?}: {adapted}");
        assert_eq!(parsed.analyses.len(), 1);
    }
}

#[test]
fn ngspice_declarative_controls_accept_semicolon_comments_without_executing_them() {
    let profile = NetlistExecutionProfile::Spice3NgspiceV2;
    let source = "title\r\nV1 out 0 1\r\nR1 out 0 1k\r\n.control; boundary\r\nop; wrdata ignored.txt v(out)\r\nsave v(out); saved output\r\n.endc; boundary\r\n.end; done\r\n";
    let adapted = profile
        .adapt_source(source)
        .expect("ngspice semicolons introduce comments");
    profile
        .validate_executable_source(&adapted)
        .expect("adapted control source is admissible");
    let parsed = rspice_core::Netlist::parse(&adapted).expect("the declarative script parses");
    assert_eq!(parsed.analyses.len(), 1);
    assert_eq!(adapted.lines().count(), source.lines().count());
    assert!(
        parsed
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "control-command-dropped")
    );
    assert!(
        profile
            .adapt_source("title\n.control\nwrdata output.txt v(out)\n.endc\n.end\n")
            .is_err()
    );
}
