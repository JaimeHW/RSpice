//! The project's signal sets as the results state holds them: the limits a
//! set keeps to, and the repair a project file's sets get when it opens.

use rspice_results::result_presentation::SignalSet;

/// The most sets a project keeps, and the most signals one set holds: a
/// project file is input like any other, and the browser's menu lists every
/// set.
pub(crate) const MAX_SIGNAL_SETS: usize = 256;
pub(crate) const MAX_SIGNAL_SET_MEMBERS: usize = 4_096;
const MAX_SIGNAL_SET_NAME_CHARS: usize = 80;

/// A set name as the reader typed it, trimmed and checked.
pub(crate) fn signal_set_name(raw: &str) -> Result<String, String> {
    let name = raw.trim();
    if name.is_empty() {
        return Err("Name the set first.".to_owned());
    }
    if name.chars().count() > MAX_SIGNAL_SET_NAME_CHARS {
        return Err(format!(
            "A set name holds at most {MAX_SIGNAL_SET_NAME_CHARS} characters."
        ));
    }
    Ok(name.to_owned())
}

/// The set named `name`, ignoring case: two sets that differ only in case
/// would read as one in the menu.
pub(crate) fn signal_set_index(sets: &[SignalSet], name: &str) -> Option<usize> {
    sets.iter()
        .position(|set| set.name.eq_ignore_ascii_case(name))
}

/// Repair the sets a project file carried: blank, overlong and duplicate
/// names and empty sets go, duplicate members collapse, and both limits hold.
pub(crate) fn sanitize_signal_sets(sets: Vec<SignalSet>) -> Vec<SignalSet> {
    let mut kept: Vec<SignalSet> = Vec::new();
    for set in sets {
        let Ok(name) = signal_set_name(&set.name) else {
            continue;
        };
        if signal_set_index(&kept, &name).is_some() || kept.len() >= MAX_SIGNAL_SETS {
            continue;
        }
        let mut members = Vec::new();
        for member in set.members {
            if members.len() < MAX_SIGNAL_SET_MEMBERS && !members.contains(&member) {
                members.push(member);
            }
        }
        if !members.is_empty() {
            kept.push(SignalSet { name, members });
        }
    }
    kept
}
