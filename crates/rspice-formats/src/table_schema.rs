//! Versioned schema JSON for an already-selected engineering table.
use serde::Serialize;

/// Borrowed identity and quantity metadata for one selected column.
pub struct SchemaColumn<'a> {
    pub id: &'a str,
    pub label: &'a str,
    pub unit: Option<&'a str>,
    pub identifier: bool,
}

/// View policy values retain their owner's existing serialization contracts.
pub struct SchemaView<S, F, G, V, I> {
    pub sort: S,
    pub filters: F,
    pub filter_grammar: G,
    pub virtualization: V,
    pub frozen_identifiers: I,
}

/// Encode selected columns in caller order, preserving explicit null metadata fields.
pub fn encode_table_schema<'a>(
    grid_id: &str,
    title: &str,
    source_revision: u64,
    logical_rows: usize,
    columns: impl IntoIterator<Item = SchemaColumn<'a>>,
    view: SchemaView<
        impl Serialize,
        impl Serialize,
        impl Serialize,
        impl Serialize,
        impl Serialize,
    >,
    include_metadata: bool,
) -> Result<String, String> {
    let mut schema = serde_json::json!({
        "schema": 1,
        "grid_id": grid_id,
        "title": title,
        "source_revision": include_metadata.then_some(source_revision),
        "logical_rows": logical_rows,
        "columns": columns.into_iter().map(|column| serde_json::json!({
            "id": column.id,
            "label": column.label,
            "unit": column.unit,
            "quantity_type": if column.unit.is_some() { "engineering-scalar" } else { "text" },
            "identifier": column.identifier,
        })).collect::<Vec<_>>(),
        "sort": null,
        "filters": null,
        "filter_grammar": null,
        "virtualization": null,
        "frozen_identifiers": null,
    });
    schema["sort"] = optional_metadata(&view.sort, include_metadata)?;
    schema["filters"] = optional_metadata(&view.filters, include_metadata)?;
    schema["filter_grammar"] = optional_metadata(&view.filter_grammar, include_metadata)?;
    schema["virtualization"] = optional_metadata(&view.virtualization, include_metadata)?;
    schema["frozen_identifiers"] = optional_metadata(&view.frozen_identifiers, include_metadata)?;
    serde_json::to_string_pretty(&schema).map_err(|error| error.to_string())
}

fn optional_metadata(value: &impl Serialize, include: bool) -> Result<serde_json::Value, String> {
    serde_json::to_value(include.then_some(value)).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Refused;
    impl Serialize for Refused {
        fn serialize<S: serde::Serializer>(&self, _serializer: S) -> Result<S::Ok, S::Error> {
            Err(serde::ser::Error::custom("view metadata refused"))
        }
    }

    #[test]
    fn omitted_metadata_is_not_serialized_and_requested_metadata_errors_are_returned() {
        let encode = |include| {
            encode_table_schema(
                "grid",
                "title",
                7,
                0,
                [],
                SchemaView {
                    sort: Refused,
                    filters: (),
                    filter_grammar: (),
                    virtualization: (),
                    frozen_identifiers: (),
                },
                include,
            )
        };
        let omitted: serde_json::Value = serde_json::from_str(&encode(false).unwrap()).unwrap();
        for key in [
            "source_revision",
            "sort",
            "filters",
            "filter_grammar",
            "virtualization",
            "frozen_identifiers",
        ] {
            assert!(
                omitted.get(key).is_some_and(serde_json::Value::is_null),
                "{key}"
            );
        }
        assert_eq!(encode(true).unwrap_err(), "view metadata refused");
    }
}
