use std::path::PathBuf;

use super::*;

/// A deck directory holding the programs `toggle` and `counter`.
fn deck_dir() -> PathBuf {
    let directory = std::env::temp_dir().join(format!("rspice-decks-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    for name in ["toggle", "counter"] {
        std::fs::write(directory.join(name), "").unwrap();
    }
    directory
}

fn resolved(source: &str) -> String {
    resolve_process_programs(source.to_owned(), Some(&deck_dir()))
}

fn program(name: &str) -> String {
    deck_dir().join(name).to_string_lossy().into_owned()
}

#[test]
fn a_relative_program_is_written_as_the_decks_directory_resolves_it() {
    let source = "deck\n.model p1 d_process(process_file=\"toggle\" clk_delay=1n)\n\
                  .model p2 d_process process_file=counter\n\
                  .model p3 d_process (process_file = 'pipe|')\n.end\n";
    let expected = format!(
        "deck\n.model p1 d_process(process_file=\"{}\" clk_delay=1n)\n\
         .model p2 d_process process_file={}\n\
         .model p3 d_process (process_file = '{}|')\n.end\n",
        program("toggle"),
        program("counter"),
        program("pipe"),
    );
    assert_eq!(resolved(source), expected);
}

#[test]
fn an_assignment_continued_across_lines_and_comments_is_resolved_where_its_value_stands() {
    let source = "deck\n.model p d_process (process_file\n* between the records\n\
                  + = 'toggle')\nAproc [clk] clk null [q] p\n.end\n";
    let expected = format!(
        "deck\n.model p d_process (process_file\n* between the records\n\
         + = '{}')\nAproc [clk] clk null [q] p\n.end\n",
        program("toggle")
    );
    assert_eq!(resolved(source), expected);
}

#[test]
fn an_instance_override_is_resolved_and_other_records_are_kept() {
    let peer = std::env::temp_dir().join("peer");
    assert!(peer.is_absolute());
    let tail = format!(
        ".param process_file=1\nR1 process_file 0 1k\n\
         .model q d_process process_file=\"{}\"\n.end\n",
        peer.to_string_lossy()
    );
    let source =
        format!("deck\nAproc [clk] clk null [q] p process_file=toggle ; process_file=x\n{tail}");
    let expected = format!(
        "deck\nAproc [clk] clk null [q] p process_file={} ; process_file=x\n{tail}",
        program("toggle")
    );
    assert_eq!(resolved(&source), expected);
}

#[test]
fn a_bare_program_no_file_beside_the_deck_answers_to_is_kept_for_path() {
    let source = "deck\n.model p1 d_process(process_file=\"rspice-mixed-corpus\")\n\
                  .model p2 d_process process_file=toggle\n.end\n";
    let expected = format!(
        "deck\n.model p1 d_process(process_file=\"rspice-mixed-corpus\")\n\
         .model p2 d_process process_file={}\n.end\n",
        program("toggle")
    );
    assert_eq!(resolved(source), expected);
}

#[test]
fn a_bare_program_is_found_on_the_search_path_and_nowhere_else() {
    let directory = std::env::temp_dir().join(format!("rspice-path-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join("peer"), "").unwrap();
    let search_path = std::env::join_paths([deck_dir(), directory.clone()]).unwrap();
    assert_eq!(
        program_on_path("peer", Some(&search_path)),
        Some(directory.join("peer"))
    );
    // The first directory holding it wins.
    assert_eq!(
        program_on_path("toggle", Some(&search_path)),
        Some(deck_dir().join("toggle"))
    );
    assert_eq!(program_on_path("absent", Some(&search_path)), None);
    assert_eq!(program_on_path("peer", None), None);
    // A path or a FIFO pair is never looked up.
    assert_eq!(program_on_path("sub/peer", Some(&search_path)), None);
    assert_eq!(program_on_path("peer|", Some(&search_path)), None);
}

#[test]
fn without_a_deck_directory_the_source_is_kept() {
    let source = "deck\n.model p d_process process_file=toggle\n.end\n";
    assert_eq!(resolve_process_programs(source.to_owned(), None), source);
}

#[test]
fn a_program_path_with_spaces_stays_one_value() {
    let dir = std::env::temp_dir().join(format!("my decks {}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("toggle"), "").unwrap();
    let source = "deck\n.model p d_process process_file=toggle\n.end\n";
    let expected = format!(
        "deck\n.model p d_process process_file=\"{}\"\n.end\n",
        dir.join("toggle").to_string_lossy()
    );
    assert_eq!(
        resolve_process_programs(source.to_owned(), Some(&dir)),
        expected
    );
}

#[test]
fn a_record_names_its_programs_and_instances() {
    let records = vec![
        (
            2,
            ".model p d_process (process_file = 'toggle|')".to_owned(),
        ),
        (3, "Aone [clk] clk null [q] p".to_owned()),
        (4, "Atwo %d [clk] clk null [r] P clk_delay = 1n".to_owned()),
        (5, "Aother [clk] clk null [s] p2".to_owned()),
        (6, "R1 q 0 1k".to_owned()),
    ];
    assert_eq!(process_programs(&records[0].1), ["toggle|"]);
    assert_eq!(process_instances(&records, &records[0].1), ["Aone", "Atwo"]);
    let instance = "Aproc [in] [out] null p process_file = worker";
    assert_eq!(process_programs(instance), ["worker"]);
    assert_eq!(process_instances(&records, instance), ["Aproc"]);
}

#[test]
fn a_program_named_absolutely_or_not_at_all_needs_no_deck() {
    let absolute = program("toggle");
    assert!(names_its_program_absolutely(&absolute));
    assert!(names_its_program_absolutely(&format!("{absolute}||")));
    assert!(names_its_program_absolutely(""));
    assert!(!names_its_program_absolutely("toggle"));
    assert!(!names_its_program_absolutely("./pipe|"));
}
