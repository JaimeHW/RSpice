//! Text variables: a sheet's title-block values lettered by name, written
//! `${NAME}`, wherever a sheet's text may name them, as KiCad's text
//! variables and Altium's special strings do. One resolver serves every
//! surface, so the canvas, the armed note and print letter one answer.

use std::borrow::Cow;

use super::*;

/// One value a sheet's text can letter by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SheetTextVariable {
    /// A title-block field, as the sheet's block letters it.
    Field(DrawingSheetTitleFieldId),
    /// The sheet's number among its drawing's sheets.
    SheetNumber,
    /// How many sheets the drawing has.
    SheetCount,
}

impl SheetTextVariable {
    /// Every variable, in the order a list offers them: the title block's
    /// own order, with the sheet's number and count after its Sheet.
    pub const ALL: [Self; DrawingSheetTitleFieldId::ALL.len() + 2] = {
        let mut all = [Self::SheetNumber; DrawingSheetTitleFieldId::ALL.len() + 2];
        let (mut field, mut index) = (0, 0);
        while field < DrawingSheetTitleFieldId::ALL.len() {
            let id = DrawingSheetTitleFieldId::ALL[field];
            all[index] = Self::Field(id);
            index += 1;
            if matches!(id, DrawingSheetTitleFieldId::Page) {
                all[index] = Self::SheetNumber;
                all[index + 1] = Self::SheetCount;
                index += 2;
            }
            field += 1;
        }
        all
    };

    /// The name text writes the variable by, inside `${` and `}`. KiCad's
    /// names where KiCad has the same value, so a drawing's text reads the
    /// same in both.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::SheetNumber => "#",
            Self::SheetCount => "##",
            Self::Field(field) => match field {
                DrawingSheetTitleFieldId::Project => "PROJECTNAME",
                DrawingSheetTitleFieldId::CellView => "CELLVIEW",
                DrawingSheetTitleFieldId::SheetTitle => "TITLE",
                DrawingSheetTitleFieldId::Page => "SHEET",
                DrawingSheetTitleFieldId::Revision => "REVISION",
                DrawingSheetTitleFieldId::PaperSize => "PAPER",
                DrawingSheetTitleFieldId::Scale => "SCALE",
                DrawingSheetTitleFieldId::DrawnBy => "CREATED_BY",
                DrawingSheetTitleFieldId::CheckedBy => "CHECKED_BY",
                DrawingSheetTitleFieldId::ApprovedBy => "APPROVED_BY",
                DrawingSheetTitleFieldId::Date => "ISSUE_DATE",
                DrawingSheetTitleFieldId::Organization => "COMPANY",
                DrawingSheetTitleFieldId::DocumentId => "DOCUMENT_ID",
                DrawingSheetTitleFieldId::Classification => "CLASSIFICATION",
                DrawingSheetTitleFieldId::DocumentType => "DOCUMENT_TYPE",
                DrawingSheetTitleFieldId::DocumentStatus => "STATUS",
                DrawingSheetTitleFieldId::Language => "LANGUAGE",
                DrawingSheetTitleFieldId::CageCode => "CAGE_CODE",
                DrawingSheetTitleFieldId::Engineer => "ENGINEER",
                DrawingSheetTitleFieldId::DrawnDate => "CREATED_DATE",
                DrawingSheetTitleFieldId::CheckedDate => "CHECKED_DATE",
                DrawingSheetTitleFieldId::EngineerDate => "ENGINEER_DATE",
                DrawingSheetTitleFieldId::ApprovedDate => "APPROVED_DATE",
                DrawingSheetTitleFieldId::ContractNumber => "CONTRACT_NUMBER",
            },
        }
    }

    /// What a list calls the variable: a field's own caption.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Field(field) => field.display_label(),
            Self::SheetNumber => "Sheet number",
            Self::SheetCount => "Sheet count",
        }
    }

    /// The variable text writes as `name`.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|variable| variable.name() == name)
    }
}

/// Every [`SheetTextVariable`]'s value on one sheet.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SheetTextVariables {
    values: BTreeMap<SheetTextVariable, String>,
}

impl SheetTextVariables {
    /// The values on a sheet of `format` whose automatic and project values
    /// are `authority_values`, as [`resolve_drawing_sheet_title_fields`]
    /// takes them. Each field reads as the block letters it, whether or not
    /// the block shows it, and the sheet's number and count are those its
    /// Sheet letters. A value that names another, such as a Title typed as
    /// `Sheet ${#} of ${##}`, reads with that value in its place.
    #[must_use]
    pub fn new(
        format: &SchematicSheetFormat,
        authority_values: &BTreeMap<DrawingSheetTitleFieldId, String>,
    ) -> Self {
        let (number, count) = authority_values
            .get(&DrawingSheetTitleFieldId::Page)
            .and_then(|page| page_number_and_count(page))
            .unwrap_or((1, 1));
        let typed: BTreeMap<SheetTextVariable, String> = SheetTextVariable::ALL
            .into_iter()
            .map(|variable| {
                let value = match variable {
                    SheetTextVariable::Field(field) => {
                        let authored = format
                            .title_block
                            .fields
                            .get(&field)
                            .map_or("", |state| state.value.as_str());
                        lettered_title_field(format, authority_values, field, authored).value
                    }
                    SheetTextVariable::SheetNumber => number.to_string(),
                    SheetTextVariable::SheetCount => count.to_string(),
                };
                (variable, value)
            })
            .collect();
        // One level: a value naming one that names another letters the
        // second's name, so nothing can name itself round in a loop.
        let values = typed
            .iter()
            .map(|(variable, value)| {
                let expanded = expand_text_variables(value, |name| {
                    SheetTextVariable::from_name(name)
                        .and_then(|named| typed.get(&named))
                        .map(|named| Cow::Borrowed(named.as_str()))
                });
                (*variable, expanded.into_owned())
            })
            .collect();
        Self { values }
    }

    /// The value of `variable`, where the sheet gives one.
    #[must_use]
    pub fn value(&self, variable: SheetTextVariable) -> Option<&str> {
        self.values.get(&variable).map(String::as_str)
    }
}

/// What a sheet's Sheet box letters: its `number` among its drawing's
/// `count` sheets. Its number and count are also `${#}` and `${##}`.
#[must_use]
pub fn drawing_sheet_page_label(
    number: impl std::fmt::Display,
    count: impl std::fmt::Display,
) -> String {
    format!("{number} of {count}")
}

/// The number and count a [`drawing_sheet_page_label`] letters.
fn page_number_and_count(label: &str) -> Option<(u32, u32)> {
    let (number, count) = label.split_once(" of ")?;
    Some((number.parse().ok()?, count.parse().ok()?))
}

/// A `${` without its closing `}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnterminatedTextVariable;

/// The names `text` writes as `${NAME}`, in the order it writes them.
pub fn text_variable_names(text: &str) -> Result<Vec<&str>, UnterminatedTextVariable> {
    let mut names = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("${") {
        let after = &rest[start + 2..];
        let end = after.find('}').ok_or(UnterminatedTextVariable)?;
        names.push(&after[..end]);
        rest = &after[end + 1..];
    }
    Ok(names)
}

/// `text` with each `${NAME}` that `lookup` gives a value for replaced by
/// that value. A name it gives none for, and a `${` never closed, stay as
/// written, as KiCad leaves a variable it cannot resolve.
pub fn expand_text_variables<'t, 'v>(
    text: &'t str,
    lookup: impl Fn(&str) -> Option<Cow<'v, str>>,
) -> Cow<'t, str> {
    if !text.contains("${") {
        return Cow::Borrowed(text);
    }
    let mut expanded = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("${") {
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            break;
        };
        expanded.push_str(&rest[..start]);
        match lookup(&after[..end]) {
            Some(value) => expanded.push_str(&value),
            None => expanded.push_str(&rest[start..start + 2 + end + 1]),
        }
        rest = &after[end + 1..];
    }
    expanded.push_str(rest);
    Cow::Owned(expanded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_field_has_its_own_name() {
        let mut names = SheetTextVariable::ALL.map(SheetTextVariable::name).to_vec();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), SheetTextVariable::ALL.len());
        for variable in SheetTextVariable::ALL {
            assert_eq!(
                SheetTextVariable::from_name(variable.name()),
                Some(variable)
            );
        }
    }

    #[test]
    fn a_name_without_a_value_stays_as_written() {
        let lookup = |name: &str| (name == "TITLE").then_some(Cow::Borrowed("Amplifier"));
        assert_eq!(
            expand_text_variables("${TITLE} rev ${REV} ${TITLE", lookup),
            "Amplifier rev ${REV} ${TITLE"
        );
        assert_eq!(
            text_variable_names("a ${TITLE} b ${X}"),
            Ok(vec!["TITLE", "X"])
        );
        assert_eq!(
            text_variable_names("${TITLE"),
            Err(UnterminatedTextVariable)
        );
    }

    /// A field reads as the block letters it, typed or supplied, whether or
    /// not the block shows it.
    #[test]
    fn fields_read_as_the_block_letters_them() {
        let mut format = SchematicSheetFormat::default();
        format
            .title_block
            .fields
            .entry(DrawingSheetTitleFieldId::SheetTitle)
            .or_default()
            .value = "Power stage".to_owned();
        let authority = BTreeMap::from([
            (DrawingSheetTitleFieldId::SheetTitle, "top".to_owned()),
            (DrawingSheetTitleFieldId::Revision, "B".to_owned()),
        ]);
        let variables = SheetTextVariables::new(&format, &authority);
        let field = |id| variables.value(SheetTextVariable::Field(id));
        assert_eq!(
            field(DrawingSheetTitleFieldId::SheetTitle),
            Some("Power stage")
        );
        assert_eq!(field(DrawingSheetTitleFieldId::Revision), Some("B"));
        assert_eq!(field(DrawingSheetTitleFieldId::Date), Some("—"));
        // A sheet with no Sheet value is the one sheet of its drawing.
        assert_eq!(variables.value(SheetTextVariable::SheetNumber), Some("1"));
        assert_eq!(variables.value(SheetTextVariable::SheetCount), Some("1"));
    }

    /// A title-block field typed with the sheet's number and count letters
    /// them, on the canvas and in print alike, since both resolve the block
    /// through here; a value naming another reads with it in place.
    #[test]
    fn a_title_field_letters_the_variables_it_names() {
        let mut format = SchematicSheetFormat::default();
        format
            .title_block
            .fields
            .entry(DrawingSheetTitleFieldId::SheetTitle)
            .or_default()
            .value = "Sheet ${#} of ${##}, rev ${REVISION}".to_owned();
        let authority = BTreeMap::from([
            (
                DrawingSheetTitleFieldId::Page,
                drawing_sheet_page_label(2, 5),
            ),
            (DrawingSheetTitleFieldId::Revision, "B".to_owned()),
        ]);
        let title = resolve_drawing_sheet_title_fields(&format, &authority)
            .into_iter()
            .find(|field| field.id == DrawingSheetTitleFieldId::SheetTitle)
            .expect("the block letters a Title")
            .value;
        assert_eq!(title, "Sheet 2 of 5, rev B");
        let variables = SheetTextVariables::new(&format, &authority);
        assert_eq!(
            variables.value(SheetTextVariable::Field(
                DrawingSheetTitleFieldId::SheetTitle
            )),
            Some("Sheet 2 of 5, rev B")
        );
        assert_eq!(variables.value(SheetTextVariable::SheetNumber), Some("2"));
    }
}
