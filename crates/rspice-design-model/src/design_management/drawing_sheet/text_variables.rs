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
}

impl SheetTextVariable {
    /// Every variable, in the order a list offers them: the title block's
    /// own order.
    pub const ALL: [Self; DrawingSheetTitleFieldId::ALL.len()] = {
        let mut all =
            [Self::Field(DrawingSheetTitleFieldId::Project); DrawingSheetTitleFieldId::ALL.len()];
        let mut index = 0;
        while index < all.len() {
            all[index] = Self::Field(DrawingSheetTitleFieldId::ALL[index]);
            index += 1;
        }
        all
    };

    /// The name text writes the variable by, inside `${` and `}`. KiCad's
    /// names where KiCad has the same value, so a drawing's text reads the
    /// same in both.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
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

    /// What a list calls the variable: the field's own caption.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Field(field) => field.display_label(),
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
    /// the block shows it.
    #[must_use]
    pub fn new(
        format: &SchematicSheetFormat,
        authority_values: &BTreeMap<DrawingSheetTitleFieldId, String>,
    ) -> Self {
        let values = SheetTextVariable::ALL
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
                };
                (variable, value)
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
    }
}
