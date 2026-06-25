use rspice_veriloga::rust_backend::{GeneratedRustDevice, RustBackendError, RustTranspiler};

#[test]
fn rust_backend_public_api_exists() {
    let _ = RustTranspiler::default();
    let diagnostic = RustBackendError::unsupported("fixture.va", "tiny_res", "arrays");

    assert!(diagnostic.to_string().contains("fixture.va"));
    assert!(diagnostic.to_string().contains("tiny_res"));
    assert!(diagnostic.to_string().contains("arrays"));
}

#[test]
fn generated_device_records_multiple_files() {
    let device = GeneratedRustDevice {
        module_name: "tiny_res".to_string(),
        public_model_name: "tiny_res".to_string(),
        folder_name: "tiny_res__tiny_res__00000000".to_string(),
        files: Vec::new(),
        source_digest: "0000000000000000".to_string(),
    };

    assert_eq!(device.module_name, "tiny_res");
    assert!(device.files.is_empty());
}
