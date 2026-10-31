use std::env;

/// Development license material is an explicit build-profile capability, not a
/// side effect of `debug_assertions` (which can be enabled for a release
/// profile). Standard debug builds retain the signed sample-key workflow and
/// the runtime cloud endpoints of a local stack; every release and custom
/// profile fails closed unless backed by a production ceremony key compiled
/// into the application. The profile the decision was made from is passed on
/// too, so a test pins the rule.
fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(rspice_development_build)");
    let profile = env::var("PROFILE").expect("Cargo must provide PROFILE");
    if profile == "debug" {
        println!("cargo::rustc-cfg=rspice_development_build");
    }
    println!("cargo::rustc-env=RSPICE_BUILD_PROFILE={profile}");
}
