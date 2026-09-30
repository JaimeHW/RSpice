//! Application integration for sealed Verilog-A project, model and PDK sources.

use rspice_simulation::netlist_preparation::{
    append_project_veriloga_directive, project_veriloga_directive,
};
use rspice_simulation::project_veriloga::{
    build_profile, compile_model_library_source_runtimes, compile_project_source_bundle_runtime,
    prepare_project_runtime,
};
use rspice_simulation::veriloga::{
    PreparedVerilogARuntime, PreparedVerilogARuntimeSet, compile_signed_pdk_source_runtime,
};

mod project_tests;
pub(crate) mod test_support;

mod tests {
    use crate::state::model_library::ModelLibraryManager;

    #[test]
    fn standalone_connection_import_preserves_sources_and_rejects_corrupt_bindings() {
        use super::{
            PreparedVerilogARuntimeSet, compile_model_library_source_runtimes, test_support::*,
        };
        let (sources, _) = standalone_connection_fixture();
        assert_eq!(sources.len(), 2);
        assert_eq!(
            sources.device_runtimes().len(),
            1,
            "only devices enter the JIT inventory"
        );
        let encoded = serde_json::to_value(&sources).unwrap();
        let mut connection_only = encoded.clone();
        connection_only["runtimes"] = serde_json::json!([]);
        let library_only: PreparedVerilogARuntimeSet =
            serde_json::from_value(connection_only).unwrap();
        library_only.validate().unwrap();
        assert!(!library_only.is_empty());
        let devices =
            PreparedVerilogARuntimeSet::try_new(sources.device_runtimes().cloned().collect())
                .unwrap();
        assert_eq!(devices.try_merge(library_only).unwrap(), sources);

        let restored: PreparedVerilogARuntimeSet = serde_json::from_value(encoded.clone()).unwrap();
        restored.validate().unwrap();
        assert_eq!(restored, sources);
        for field in ["source_key", "netlist_alias", "source_digest"] {
            let mut corrupt = encoded.clone();
            corrupt["connections"][0][field] = serde_json::json!("altered");
            let decoded = serde_json::from_value::<PreparedVerilogARuntimeSet>(corrupt);
            assert!(
                decoded.is_err() || decoded.unwrap().validate().is_err(),
                "{field}"
            );
        }
        let mut corrupt = encoded.clone();
        corrupt["connections"][0]["artifact"]["preprocessed_source"] =
            serde_json::json!("connectrules Changed; endconnectrules");
        assert!(
            serde_json::from_value::<PreparedVerilogARuntimeSet>(corrupt)
                .unwrap()
                .validate()
                .is_err()
        );
        let mut old_shape = encoded;
        old_shape.as_object_mut().unwrap().remove("connections");
        assert!(serde_json::from_value::<PreparedVerilogARuntimeSet>(old_shape).is_err());

        let authority =
            standalone_connection_authority_with_directive(".va \"rules.vams\" ui_driver\n");
        assert!(
            compile_model_library_source_runtimes(&authority)
                .unwrap_err()
                .to_string()
                .contains("different prepared artifacts")
        );
        let authority = standalone_connection_authority_with_directive(".va \"rules.vams\"\n");
        let unnamed = compile_model_library_source_runtimes(&authority).unwrap();
        assert!(
            unnamed
                .sources()
                .any(|source| source.netlist_alias().starts_with("__rspice_connections_"))
        );
        let authority =
            standalone_connection_authority_with_directive(".va \"rules.vams\" module=ui_driver\n");
        assert!(
            compile_model_library_source_runtimes(&authority)
                .unwrap_err()
                .to_string()
                .contains("does not declare module")
        );
    }

    #[test]
    fn sealed_library_module_selection_keeps_aliases_and_module_identity() {
        fn selected_authority(
            first_module: &str,
        ) -> rspice_simulation::model_sources::SealedModelLibraryVerilogAAuthority {
            let mut manager = ModelLibraryManager::new();
            manager.load_library_bundle(
                "selected-modules.lib",
                vec![
                    ("root.lib".to_owned(), format!(".va \"devices.va\" first_alias module={first_module}\n.va \"devices.va\" second_alias module=second_load\n.model fallback_d D\n").into_bytes()),
                    ("devices.va".to_owned(), b"module first_load(p,n); inout p,n; electrical p,n; analog I(p,n)<+V(p,n); endmodule\nmodule second_load(p,n); inout p,n; electrical p,n; analog I(p,n)<+2*V(p,n); endmodule\n".to_vec()),
                ],
                None,
            ).unwrap();
            let sealed = manager.seal_execution_sources().unwrap();
            sealed.model_library_veriloga_authority().unwrap().unwrap()
        }
        let authority = selected_authority("first_load");
        assert_eq!(authority.roots().len(), 2);
        assert_eq!(
            authority.roots()[0].selected_module.as_deref(),
            Some("first_load")
        );
        assert_eq!(
            authority.roots()[1].selected_module.as_deref(),
            Some("second_load")
        );
        let runtimes =
            rspice_simulation::project_veriloga::compile_model_library_source_runtimes(&authority)
                .unwrap();
        let aliases = runtimes
            .device_runtimes()
            .map(|runtime| runtime.netlist_alias())
            .collect::<Vec<_>>();
        assert_eq!(aliases, ["first_alias", "second_alias"]);
        let keys = runtimes
            .device_runtimes()
            .map(|runtime| runtime.source_key())
            .collect::<Vec<_>>();
        assert_ne!(keys[0], keys[1]);
        let missing = selected_authority("FIRST_LOAD");
        let error =
            rspice_simulation::project_veriloga::compile_model_library_source_runtimes(&missing)
                .unwrap_err();
        assert!(
            error.to_string().contains("does not declare module"),
            "{error}"
        );
    }
}
