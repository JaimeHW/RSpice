//! Emit the executable built-in XSPICE interface registry in a reviewable form.

use rspice_core::xspice::CodeModelRegistry;

fn main() {
    let registry = CodeModelRegistry::with_builtins();
    for model_name in registry.model_names() {
        let model = registry
            .get(model_name)
            .expect("a name returned by the registry must resolve");
        println!("MODEL {model_name}");
        for port in model.ports() {
            println!(
                "  PORT {} {:?} {:?} vector={} null={} min={:?} max={:?}",
                port.name,
                port.direction,
                port.default_type,
                port.is_vector,
                port.null_allowed,
                port.vector_min_len,
                port.vector_max_len
            );
        }
        for parameter in model.parameters() {
            println!(
                "  PARAM {} {:?} required={} min={:?} max={:?}",
                parameter.name,
                parameter.param_type,
                parameter.required,
                parameter.min,
                parameter.max
            );
        }
    }
}
