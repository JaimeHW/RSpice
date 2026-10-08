use super::*;

fn floating_fields(value: &serde_json::Value, path: &str, fields: &mut Vec<String>) {
    match value {
        serde_json::Value::Number(number) if number.is_f64() => fields.push(path.to_owned()),
        serde_json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                floating_fields(value, &format!("{path}/{index}"), fields);
            }
        }
        serde_json::Value::Object(values) => {
            for (key, value) in values {
                let key = key.replace('~', "~0").replace('/', "~1");
                floating_fields(value, &format!("{path}/{key}"), fields);
            }
        }
        _ => {}
    }
}

#[test]
fn every_result_family_rejects_numeric_loss_in_floating_fields() {
    let mut checked = 0;
    for kind in AnalysisResultKind::ALL {
        let document = document_for(kind);
        let mut value = serde_json::to_value(document).unwrap();
        let mut fields = Vec::new();
        floating_fields(&value, "", &mut fields);
        assert!(
            !fields.is_empty(),
            "{} fixture needs floating-point evidence",
            kind.tag()
        );
        for field in fields {
            let original = value.pointer(&field).unwrap().clone();
            for (literal, diagnostic) in [
                ("1e-999", "underflow"),
                ("9007199254740993", "cannot be represented exactly"),
            ] {
                *value.pointer_mut(&field).unwrap() = serde_json::json!("NUMERIC_TOKEN");
                let json = serde_json::to_string(&value)
                    .unwrap()
                    .replace("\"NUMERIC_TOKEN\"", literal);
                let error = AnalysisResultDocument::from_json(&json).unwrap_err();
                assert!(
                    error.to_string().contains(diagnostic),
                    "{}/{field}/{literal}: {error}",
                    kind.tag()
                );
                checked += 1;
            }
            *value.pointer_mut(&field).unwrap() = original;
        }
    }
    eprintln!(
        "Rejected numeric loss at {checked} floating-point field/literal combinations across every result family"
    );
}
