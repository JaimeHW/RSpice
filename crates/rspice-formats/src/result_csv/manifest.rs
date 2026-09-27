//! CSV schema for the canonical retained dataset manifest projection.
use crate::table::escape_csv_field as csv_field;
use rspice_results::manifest::ManifestViewModel;

/// Encode the projected integrity, authority and inventory exactly as retained.
pub fn encode_manifest_csv(manifest: &ManifestViewModel) -> String {
    let mut contents = String::from(
        "section,field,value,analysis,expansion,tasks,domain_axis,stored_values,precision,eligibility,task_identity,config_digest\n",
    );
    {
        let mut metadata = |section: &str, field: &str, value: &str| {
            contents.push_str(&format!(
                "{},{},{},,,,,,,,,\n",
                csv_field(section),
                csv_field(field),
                csv_field(value)
            ));
        };
        for (field, value) in [
            ("dataset_id", manifest.dataset_id.as_str()),
            ("dataset_digest", manifest.dataset_digest.as_str()),
            ("run_id", manifest.run_id.as_str()),
            ("run_sequence", manifest.run_sequence.as_str()),
            ("run_label", manifest.run_label.as_str()),
            ("lifecycle", manifest.lifecycle.as_str()),
            ("execution_target", manifest.execution_target.as_str()),
            ("elapsed_time", manifest.elapsed_time.as_str()),
            ("inventory_title", manifest.inventory_title.as_str()),
            ("inventory_status", manifest.inventory_status.as_str()),
            ("integrity", manifest.integrity.as_str()),
            ("qualification", manifest.qualification.as_str()),
        ] {
            metadata("dataset", field, value);
        }
        metadata("dataset", "task_count", &manifest.task_count.to_string());
        metadata(
            "dataset",
            "retained_result_count",
            &manifest.retained_result_count.to_string(),
        );
    }
    for row in &manifest.rows {
        contents.push_str(&format!(
            "inventory,,,{},{},{},{},{},{},{},{},{}\n",
            csv_field(&row.analysis),
            csv_field(&row.expansion),
            csv_field(&row.tasks),
            csv_field(&row.domain_axis),
            csv_field(&row.stored_values),
            csv_field(&row.precision),
            csv_field(&row.eligibility),
            csv_field(row.task_identity.as_deref().unwrap_or_default()),
            csv_field(row.config_digest.as_deref().unwrap_or_default()),
        ));
    }

    if let Some(authority) = &manifest.authority {
        let mut authority_field = |field: &str, value: &str| {
            contents.push_str(&format!(
                "authority,{},{},,,,,,,,,\n",
                csv_field(field),
                csv_field(value)
            ));
        };
        for (field, value) in [
            ("source_domain", authority.source_domain.as_str()),
            (
                "simulation_plan_id",
                authority.simulation_plan_id.as_deref().unwrap_or_default(),
            ),
            ("project_revision", authority.project_revision.as_str()),
            (
                "prepared_snapshot_digest",
                authority.prepared_snapshot_digest.as_str(),
            ),
            (
                "source_content_digest",
                authority.source_content_digest.as_str(),
            ),
            ("source_check", authority.source_check.as_str()),
            (
                "source_check_digest",
                authority.source_check_digest.as_str(),
            ),
        ] {
            authority_field(field, value);
        }
        for (name, digest) in &authority.model_sources {
            authority_field(&format!("model_source:{name}"), digest);
        }
    }

    contents
}
