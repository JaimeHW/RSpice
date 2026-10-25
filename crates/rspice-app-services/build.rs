use std::env;

/// The managed Automation runtime's verification key is compiled in where it
/// is verified. It is a trust anchor: a key installed at run time could be
/// replaced, and an unset one must never be a state the verifier can read.
/// Release builds supply both values together; ordinary builds supply neither,
/// and the runtime then refuses every managed environment.
fn main() {
    println!("cargo:rerun-if-env-changed=RSPICE_AUTOMATION_RUNTIME_KEY_ID");
    println!("cargo:rerun-if-env-changed=RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX");
    let runtime_key_id = env::var("RSPICE_AUTOMATION_RUNTIME_KEY_ID").unwrap_or_default();
    let runtime_public_key =
        env::var("RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX").unwrap_or_default();
    if runtime_key_id.is_empty() != runtime_public_key.is_empty() {
        panic!(
            "RSPICE_AUTOMATION_RUNTIME_KEY_ID and RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX must be supplied together"
        );
    }
    if !runtime_public_key.is_empty()
        && (runtime_public_key.len() != 64
            || !runtime_public_key
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit()))
    {
        panic!(
            "RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX must contain exactly 64 hexadecimal digits"
        );
    }
    println!("cargo:rustc-env=RSPICE_AUTOMATION_RUNTIME_KEY_ID={runtime_key_id}");
    println!("cargo:rustc-env=RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX={runtime_public_key}");
}
