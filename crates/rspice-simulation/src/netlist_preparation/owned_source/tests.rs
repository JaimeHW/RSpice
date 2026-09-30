use super::*;

#[test]
fn narrow_override_reaches_execution_before_the_first_commented_termination() {
    let base = ".end\r\nR1 1 0 1k\r\n.end; first\r\n.end\r\n";
    let source =
        insert_before_end(base, ".options reltol=0.012345").expect("base has a terminator");
    let parsed = rspice_core::Netlist::parse(&source).expect("composed source parses");
    assert_eq!(parsed.title, ".end");
    assert_eq!(parsed.options.reltol, Some(0.012345), "{source}");
    assert!(source.starts_with(".end\r\nR1 1 0 1k\r\n"), "{source:?}");
    assert!(source.ends_with(".end; first\r\n.end\r\n"), "{source:?}");
    assert!(insert_before_end(".end\nR1 1 0 1k", ".op").is_err());
}
