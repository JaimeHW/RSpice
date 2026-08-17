//! Importing design variables from a spec-sheet CSV.
//!
//! Design variables arrive from a spec sheet, and typing forty of them one
//! dialog at a time is not an authoring workflow. The import is nonetheless
//! held to the same contract as a hand-typed variable: every row becomes a
//! [`DesignVariableDraft`] and clears `design_variable_from_draft`, so there is
//! one definition of a valid variable rather than a second, laxer one reached
//! through a file.
//!
//! The sheet is the transaction. A row that fails names its line and the whole
//! import is refused: a partly applied spec sheet would leave a registry no one
//! authored, and the engineer would have to work out which rows landed.

#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

use csv::{ReaderBuilder, StringRecord, Trim};

use crate::product::SimulationPlanId;
use crate::state::{
    DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity,
    DesignVariableSweepEligibility,
};
use crate::workbench::RSpiceApp;
use crate::workbench::state::DesignVariableDraft;

use super::workflows::{commit_plan_change, design_variable_from_draft};

/// A spec sheet is a table of typed parameters, not a data file. The ceiling
/// bounds a mis-picked waveform dump before any of it is decoded.
const MAX_IMPORT_BYTES: usize = 1 << 20;

/// Row ceiling. A registry larger than this is generated, not authored, and
/// belongs in a project the plan includes rather than in one paste.
const MAX_IMPORT_ROWS: usize = 512;

/// The columns a spec sheet may name.
///
/// A header row binds each column to a field, so the order they appear in the
/// file is not load-bearing. Only the first three are required: the rest fall
/// back to the values the create dialog opens with, which is what makes a
/// three-column sheet a complete import rather than a partial one.
const COLUMNS: [&str; 9] = [
    "name",
    "quantity",
    "expression",
    "minimum",
    "maximum",
    "scope",
    "sweep role",
    "override policy",
    "description",
];

const NAME: usize = 0;
const QUANTITY: usize = 1;
const EXPRESSION: usize = 2;
const MINIMUM: usize = 3;
const MAXIMUM: usize = 4;
const SCOPE: usize = 5;
const SWEEP_ROLE: usize = 6;
const OVERRIDE_POLICY: usize = 7;
const DESCRIPTION: usize = 8;

const REQUIRED: [usize; 3] = [NAME, QUANTITY, EXPRESSION];

/// The ownership scopes, spelled as `design_variable_from_draft` indexes them.
/// `cell` and `analysis` resolve against the active view and the selected
/// analysis, so an analysis-scoped row is refused when nothing is selected —
/// the same refusal the create dialog gives.
///
/// Unlike the other enumerated columns these cannot be derived from an `ALL`
/// array: two of the four variants carry payloads, so there is no value to call
/// `label()` on without inventing a cell reference. The spellings are therefore
/// copied, and `scope_spellings_match_the_scopes_they_name` pins the copy to the
/// original — a renamed scope must fail that test rather than silently stop
/// matching the column an engineer wrote.
const SCOPE_CHOICES: [(&str, usize); 7] = [
    ("Lab characterization · testbench", 0),
    ("testbench", 0),
    ("Project", 1),
    ("Selected cell", 2),
    ("cell", 2),
    ("Selected analysis only", 3),
    ("analysis", 3),
];

/// Import a spec sheet: one file pick, one parse, one transaction.
pub(super) fn import_from_file(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
) {
    match pick_spec_sheet(ctx) {
        Ok(Some((file_name, source))) => commit_import(app, plan_id, &file_name, &source),
        Ok(None) => {}
        Err(error) => refuse(app, &error),
    }
}

/// Adopt a spec sheet the browser's picker has finished reading.
///
/// A browser cannot hand back a file synchronously, so the click only starts
/// the read and a later frame collects it. The pending read is held in the
/// frame context beside the surface that started it: it belongs to this
/// session's picker, never to the project.
#[cfg(target_arch = "wasm32")]
pub(super) fn poll_pending_import(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    plan_id: SimulationPlanId,
) {
    let Some(mailbox) = ctx.data(|data| data.get_temp::<ImportMailbox>(mailbox_id())) else {
        return;
    };
    let Some(result) = mailbox.lock().ok().and_then(|mut slot| slot.take()) else {
        return;
    };
    ctx.data_mut(|data| data.remove::<ImportMailbox>(mailbox_id()));
    match result {
        Ok((file_name, source)) => commit_import(app, plan_id, &file_name, &source),
        Err(error) => refuse(app, &error),
    }
}

/// Parse, validate and adopt one spec sheet as a single plan transaction.
///
/// `commit_plan_change` owns the transaction, so the import produces exactly
/// one configuration receipt and one preflight invalidation, and a refusal by
/// the workspace leaves the plan as it was.
fn commit_import(app: &mut RSpiceApp, plan_id: SimulationPlanId, file_name: &str, source: &str) {
    let variables = match design_variables_from_csv(app, source) {
        Ok(variables) => variables,
        Err(error) => return refuse(app, &error),
    };
    let detail = format!(
        "Imported {} design variable{} from {file_name}.",
        variables.len(),
        if variables.len() == 1 { "" } else { "s" }
    );
    commit_plan_change(app, plan_id, &detail, move |workspace, plan_id| {
        workspace
            .add_design_variables(plan_id, variables)
            .map_err(|error| error.to_string())
    });
}

/// Report a refused import where a refused edit is already reported: the plan's
/// lifecycle status, which is the channel `commit_plan_change` itself writes on
/// failure.
fn refuse(app: &mut RSpiceApp, error: &str) {
    app.state
        .workbench
        .analysis_lifecycle_status
        .record_refusal(format!("Design-variable import refused · {error}"));
}

/// Parse a spec sheet and put every row through the create dialog's contract.
fn design_variables_from_csv(app: &RSpiceApp, source: &str) -> Result<Vec<DesignVariable>, String> {
    parse_design_variable_csv(source)?
        .into_iter()
        .map(|(line, draft)| {
            design_variable_from_draft(app, &draft)
                .map_err(|error| format!("line {line} · {error}"))
        })
        .collect()
}

/// Parse a spec sheet into one draft per row, keeping each row's line number.
///
/// The line is the file's own, counted the way an editor counts it, so a
/// refusal points at the row the engineer has to fix.
fn parse_design_variable_csv(source: &str) -> Result<Vec<(u64, DesignVariableDraft)>, String> {
    if source.len() > MAX_IMPORT_BYTES {
        return Err(format!(
            "the spec sheet exceeds the {MAX_IMPORT_BYTES} byte import limit"
        ));
    }
    // Spreadsheets write a byte-order mark, which would otherwise become part
    // of the first column's name and leave `name` unrecognized.
    let source = source.strip_prefix('\u{feff}').unwrap_or(source);
    let mut reader = ReaderBuilder::new()
        .trim(Trim::All)
        .flexible(false)
        .from_reader(source.as_bytes());
    let headers = reader.headers().map_err(malformed_csv)?.clone();
    let columns = header_map(&headers)?;

    let mut rows = Vec::new();
    for record in reader.records() {
        if rows.len() >= MAX_IMPORT_ROWS {
            return Err(format!(
                "the spec sheet exceeds the {MAX_IMPORT_ROWS} row import limit"
            ));
        }
        let record = record.map_err(malformed_csv)?;
        let line = record.position().map_or(0, csv::Position::line);
        let draft =
            draft_from_row(&record, &columns).map_err(|error| format!("line {line} · {error}"))?;
        rows.push((line, draft));
    }
    if rows.is_empty() {
        return Err("the spec sheet names its columns but holds no variables".to_owned());
    }
    Ok(rows)
}

fn malformed_csv(error: csv::Error) -> String {
    format!("the spec sheet is not valid CSV · {error}")
}

/// Bind each declared column to the field index it occupies.
///
/// An unrecognized header is refused rather than ignored: a sheet that spells
/// the bound column `min` would otherwise import silently unbounded variables,
/// and nothing on the page would say the limits had been dropped.
fn header_map(headers: &StringRecord) -> Result<[Option<usize>; COLUMNS.len()], String> {
    let mut map = [None; COLUMNS.len()];
    for (field, header) in headers.iter().enumerate() {
        let column = COLUMNS
            .iter()
            .position(|column| same_column(column, header))
            .ok_or_else(|| {
                format!(
                    "line 1 · the first row must name the columns, and '{header}' is not one of: {}",
                    COLUMNS.join(", ")
                )
            })?;
        if map[column].replace(field).is_some() {
            return Err(format!(
                "line 1 · column '{}' is named twice",
                COLUMNS[column]
            ));
        }
    }
    let missing = REQUIRED
        .iter()
        .filter(|column| map[**column].is_none())
        .map(|column| COLUMNS[*column])
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(map)
    } else {
        Err(format!(
            "line 1 · the header row must name {}",
            missing.join(", ")
        ))
    }
}

/// Header names are matched on their letters and digits alone, so `Sweep role`,
/// `sweep_role` and `SweepRole` all bind the same column.
fn same_column(column: &str, header: &str) -> bool {
    fn key(text: &str) -> String {
        text.chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|character| character.to_ascii_lowercase())
            .collect()
    }
    key(column) == key(header)
}

fn draft_from_row(
    row: &StringRecord,
    columns: &[Option<usize>; COLUMNS.len()],
) -> Result<DesignVariableDraft, String> {
    let cell = |column: usize| {
        columns[column]
            .and_then(|field| row.get(field))
            .unwrap_or_default()
            .trim()
    };
    let name = cell(NAME);
    let expression = cell(EXPRESSION);
    if name.is_empty() {
        return Err("name is required".to_owned());
    }
    if expression.is_empty() {
        return Err("expression is required".to_owned());
    }
    Ok(DesignVariableDraft {
        name: name.to_owned(),
        expression: expression.to_owned(),
        quantity: enumerated("quantity", cell(QUANTITY), &quantity_choices(), None)?,
        scope: enumerated("scope", cell(SCOPE), &SCOPE_CHOICES, Some(0))?,
        description: cell(DESCRIPTION).to_owned(),
        allowed_range: allowed_range(cell(MINIMUM), cell(MAXIMUM))?,
        sweep_eligibility: enumerated(
            "sweep role",
            cell(SWEEP_ROLE),
            &sweep_role_choices(),
            Some(0),
        )?,
        override_policy: enumerated(
            "override policy",
            cell(OVERRIDE_POLICY),
            &override_policy_choices(),
            Some(0),
        )?,
        validation_error: None,
    })
}

/// Compose the draft's `minimum … maximum` bound from the two columns.
///
/// One bound without the other is refused rather than half-applied: the record
/// editor enforces the same rule, because a variable with only a floor is not
/// something preflight can check.
fn allowed_range(minimum: &str, maximum: &str) -> Result<String, String> {
    match (minimum.is_empty(), maximum.is_empty()) {
        (true, true) => Ok(String::new()),
        (false, false) => {
            for (column, bound) in [("minimum", minimum), ("maximum", maximum)] {
                if bound.contains('…') || bound.contains("..") {
                    return Err(format!(
                        "{column} '{bound}' contains a range separator; give each bound its own column"
                    ));
                }
            }
            Ok(format!("{minimum} … {maximum}"))
        }
        _ => Err("a bounded variable needs both a minimum and a maximum".to_owned()),
    }
}

/// Resolve one enumerated cell to the index its draft field holds.
///
/// A cell may carry the label the studio shows or a short token, matched
/// ignoring case. An empty cell takes the default; a column with no default is
/// required.
fn enumerated(
    column: &str,
    cell: &str,
    choices: &[(&str, usize)],
    default: Option<usize>,
) -> Result<usize, String> {
    if cell.is_empty() {
        return default.ok_or_else(|| format!("{column} is required"));
    }
    choices
        .iter()
        .find(|(spelling, _)| spelling.eq_ignore_ascii_case(cell))
        .map(|(_, index)| *index)
        .ok_or_else(|| {
            format!(
                "{column} '{cell}' is not one of: {}",
                choices
                    .iter()
                    .map(|(spelling, _)| *spelling)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
}

/// Every label paired with its draft index, then the short spellings a spec
/// sheet is likely to carry instead of a phrase with punctuation in it.
///
/// Deriving the indices from the label array is what keeps the accepted
/// spellings from drifting out of step with the options the studio offers.
fn spellings(
    labels: &[&'static str],
    aliases: &[(&'static str, usize)],
) -> Vec<(&'static str, usize)> {
    labels
        .iter()
        .enumerate()
        .map(|(index, label)| (*label, index))
        .chain(aliases.iter().copied())
        .collect()
}

/// Quantity labels are single words already, so they need no short form.
fn quantity_choices() -> Vec<(&'static str, usize)> {
    spellings(
        &DesignVariableQuantity::ALL.map(DesignVariableQuantity::label),
        &[],
    )
}

fn sweep_role_choices() -> Vec<(&'static str, usize)> {
    spellings(
        &DesignVariableSweepEligibility::ALL.map(DesignVariableSweepEligibility::label),
        &[("sweep", 0), ("optimization", 1), ("fixed", 2)],
    )
}

fn override_policy_choices() -> Vec<(&'static str, usize)> {
    spellings(
        &DesignVariableOverridePolicy::ALL.map(DesignVariableOverridePolicy::label),
        &[("override", 0), ("inherit", 1)],
    )
}

/// The desktop picker hands back the file's bytes directly.
#[cfg(not(target_arch = "wasm32"))]
fn pick_spec_sheet(_ctx: &egui::Context) -> Result<Option<(String, String)>, String> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("Design variable spec sheet", &["csv"])
        .pick_file()
    else {
        return Ok(None);
    };
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("the spec sheet")
        .to_owned();
    read_bounded_utf8(&path).map(|source| Some((file_name, source)))
}

#[cfg(not(target_arch = "wasm32"))]
fn read_bounded_utf8(path: &Path) -> Result<String, String> {
    use std::io::Read as _;

    let file = std::fs::File::open(path)
        .map_err(|error| format!("could not open the spec sheet: {error}"))?;
    let mut bytes = Vec::new();
    file.take((MAX_IMPORT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("could not read the spec sheet: {error}"))?;
    if bytes.len() > MAX_IMPORT_BYTES {
        return Err(format!(
            "the spec sheet exceeds the {MAX_IMPORT_BYTES} byte import limit"
        ));
    }
    String::from_utf8(bytes).map_err(|_| "the spec sheet must be UTF-8 CSV".to_owned())
}

#[cfg(target_arch = "wasm32")]
type ImportMailbox = std::sync::Arc<std::sync::Mutex<Option<Result<(String, String), String>>>>;

#[cfg(target_arch = "wasm32")]
fn mailbox_id() -> egui::Id {
    egui::Id::new("simulation.variables.import")
}

/// The browser picker resolves on its own task; the click only starts it.
#[cfg(target_arch = "wasm32")]
fn pick_spec_sheet(ctx: &egui::Context) -> Result<Option<(String, String)>, String> {
    if ctx
        .data(|data| data.get_temp::<ImportMailbox>(mailbox_id()))
        .is_some()
    {
        return Err("a spec-sheet file picker is already open".to_owned());
    }
    let mailbox = ImportMailbox::default();
    ctx.data_mut(|data| data.insert_temp(mailbox_id(), mailbox.clone()));
    let repaint = ctx.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let result = match rfd::AsyncFileDialog::new()
            .add_filter("Design variable spec sheet", &["csv"])
            .pick_file()
            .await
        {
            Some(file) => {
                let file_name = file.file_name();
                let bytes = file.read().await;
                if bytes.len() > MAX_IMPORT_BYTES {
                    Err(format!(
                        "the spec sheet exceeds the {MAX_IMPORT_BYTES} byte import limit"
                    ))
                } else {
                    String::from_utf8(bytes)
                        .map(|source| (file_name, source))
                        .map_err(|_| "the spec sheet must be UTF-8 CSV".to_owned())
                }
            }
            None => Err("the spec-sheet import was cancelled".to_owned()),
        };
        if let Ok(mut slot) = mailbox.lock() {
            *slot = Some(result);
        }
        repaint.request_repaint();
    });
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHEET: &str = "name,quantity,expression,minimum,maximum,scope,sweep role,override policy,description\n\
         RLOAD,Resistance,10 kohm,1 kohm,1 Mohm,project,sweep,override,Output load\n\
         CFILT,Capacitance,4.7 nF,1 nF,10 nF,testbench,fixed,inherit,Filter cap\n\
         VBIAS,Voltage,1.8 V,,,,,,\n";

    fn plan(app: &RSpiceApp) -> SimulationPlanId {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("the test instance owns a stable plan")
            .id()
    }

    fn registry(app: &RSpiceApp, plan_id: SimulationPlanId) -> Vec<String> {
        app.state
            .workspace
            .plan_data(plan_id)
            .map(|payload| {
                payload
                    .design_variables
                    .iter()
                    .map(|variable| variable.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn revision(app: &RSpiceApp) -> u64 {
        app.state
            .sim_setup
            .stable_analysis_plan()
            .expect("the test instance owns a stable plan")
            .revision()
            .get()
    }

    #[test]
    fn a_well_formed_sheet_lands_as_one_transaction() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let before = revision(&app);

        commit_import(&mut app, plan_id, "loads.csv", SHEET);

        assert_eq!(registry(&app, plan_id), ["RLOAD", "CFILT", "VBIAS"]);
        assert_eq!(revision(&app), before + 1);
        let imported = &app
            .state
            .workspace
            .plan_data(plan_id)
            .expect("plan payload")
            .design_variables[0];
        assert_eq!(imported.quantity, DesignVariableQuantity::Resistance);
        assert_eq!(imported.scope, crate::state::DesignVariableScope::Project);
        assert_eq!(
            imported.sweep_eligibility,
            DesignVariableSweepEligibility::NestedSweepAndOptimization
        );
        assert_eq!(imported.description, "Output load");
        assert_eq!(
            imported
                .allowed_range
                .as_ref()
                .map(|range| (range.minimum.as_str(), range.maximum.as_str())),
            Some(("1 kohm", "1 Mohm"))
        );
        // The unbounded row is a real choice, not a missing value.
        assert!(
            app.state
                .workspace
                .plan_data(plan_id)
                .expect("plan payload")
                .design_variables[2]
                .allowed_range
                .is_none()
        );
    }

    #[test]
    fn an_out_of_range_row_names_its_line_and_imports_nothing() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let before = serde_json::to_value(&app.state.workspace).expect("workspace serializes");
        let revision_before = revision(&app);
        let sheet = "name,quantity,expression,minimum,maximum\n\
             RLOAD,Resistance,10 kohm,1 kohm,1 Mohm\n\
             RBIAS,Resistance,2 Mohm,1 kohm,1 Mohm\n\
             RTERM,Resistance,50 ohm,,\n";

        commit_import(&mut app, plan_id, "loads.csv", sheet);

        let outcome = &app.state.workbench.analysis_lifecycle_status;
        let status = outcome.message();
        assert!(outcome.is_refusal(), "{status}");
        assert!(status.contains("line 3"), "{status}");
        assert!(
            status.contains("outside the inclusive allowed range"),
            "{status}"
        );
        assert_eq!(
            serde_json::to_value(&app.state.workspace).expect("workspace still serializes"),
            before
        );
        assert_eq!(revision(&app), revision_before);
    }

    #[test]
    fn a_name_the_plan_already_owns_is_refused_atomically() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        commit_import(
            &mut app,
            plan_id,
            "loads.csv",
            "name,quantity,expression\nRLOAD,Resistance,10 kohm\n",
        );
        let before = serde_json::to_value(&app.state.workspace).expect("workspace serializes");
        let revision_before = revision(&app);

        commit_import(
            &mut app,
            plan_id,
            "more-loads.csv",
            "name,quantity,expression\nRTERM,Resistance,50 ohm\nrload,Resistance,22 kohm\n",
        );

        let outcome = &app.state.workbench.analysis_lifecycle_status;
        let status = outcome.message();
        assert!(outcome.is_refusal(), "{status}");
        assert!(status.contains("rload"), "{status}");
        assert_eq!(
            serde_json::to_value(&app.state.workspace).expect("workspace still serializes"),
            before
        );
        assert_eq!(revision(&app), revision_before);
    }

    #[test]
    fn a_name_repeated_within_the_sheet_is_refused_atomically() {
        let mut app = RSpiceApp::test_instance();
        let plan_id = plan(&app);
        let before = serde_json::to_value(&app.state.workspace).expect("workspace serializes");

        commit_import(
            &mut app,
            plan_id,
            "loads.csv",
            "name,quantity,expression\nRLOAD,Resistance,10 kohm\nRLOAD,Resistance,22 kohm\n",
        );

        let outcome = &app.state.workbench.analysis_lifecycle_status;
        assert!(outcome.is_refusal(), "{}", outcome.message());
        assert!(outcome.message().contains("RLOAD"), "{}", outcome.message());
        assert_eq!(
            serde_json::to_value(&app.state.workspace).expect("workspace still serializes"),
            before
        );
    }

    #[test]
    fn column_order_is_not_load_bearing() {
        let ordered = parse_design_variable_csv(
            "name,quantity,expression,minimum,maximum,description\n\
             RLOAD,Resistance,10 kohm,1 kohm,1 Mohm,Output load\n",
        )
        .expect("the ordered sheet parses");
        let shuffled = parse_design_variable_csv(
            "Description,MAXIMUM,expression,Quantity,minimum,Name\n\
             Output load,1 Mohm,10 kohm,Resistance,1 kohm,RLOAD\n",
        )
        .expect("the shuffled sheet parses");

        let fields = |rows: &[(u64, DesignVariableDraft)]| {
            rows.iter()
                .map(|(_, draft)| {
                    (
                        draft.name.clone(),
                        draft.expression.clone(),
                        draft.quantity,
                        draft.allowed_range.clone(),
                        draft.description.clone(),
                    )
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(fields(&ordered), fields(&shuffled));
    }

    #[test]
    fn a_misspelled_column_is_refused_rather_than_dropped() {
        let error = parse_design_variable_csv(
            "name,quantity,expression,min\nRLOAD,Resistance,10 kohm,1 kohm\n",
        )
        .expect_err("an unrecognized column is refused");
        assert!(error.contains("line 1"), "{error}");
        assert!(error.contains("'min'"), "{error}");
    }

    #[test]
    fn one_bound_without_the_other_is_refused() {
        let error = parse_design_variable_csv(
            "name,quantity,expression,minimum,maximum\nRLOAD,Resistance,10 kohm,1 kohm,\n",
        )
        .expect_err("a half-declared bound is refused");
        assert!(error.contains("line 2"), "{error}");
        assert!(error.contains("both a minimum and a maximum"), "{error}");
    }

    /// `SCOPE_CHOICES` copies the scope labels because two variants carry
    /// payloads and cannot be enumerated. This is the pin that keeps the copy
    /// honest: rename a scope and the spelling a spec sheet is allowed to use
    /// must be renamed with it, rather than quietly ceasing to match.
    #[test]
    fn scope_spellings_match_the_scopes_they_name() {
        use crate::state::DesignVariableScope;

        let app = RSpiceApp::test_instance();
        let scopes = [
            DesignVariableScope::Testbench,
            DesignVariableScope::Project,
            DesignVariableScope::SelectedCell {
                cell: app.state.workspace.active_view.clone(),
            },
            DesignVariableScope::SelectedAnalysis {
                analysis_id: crate::product::AnalysisInstanceId::new(),
            },
        ];
        for (index, scope) in scopes.iter().enumerate() {
            assert!(
                SCOPE_CHOICES
                    .iter()
                    .any(|(spelling, choice)| *choice == index && *spelling == scope.label()),
                "scope {index} is labelled {:?}, which no spec-sheet spelling accepts",
                scope.label()
            );
        }
    }

    #[test]
    fn a_sheet_with_only_a_header_row_imports_nothing() {
        assert!(
            parse_design_variable_csv("name,quantity,expression\n")
                .expect_err("a sheet with no rows is refused")
                .contains("holds no variables")
        );
    }
}
