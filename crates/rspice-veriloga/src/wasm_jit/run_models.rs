//! The browser JIT models one run asks for, and the ones it was given.
//!
//! The browser compiles and installs a run's secondary modules
//! asynchronously, before the run starts, and a device's model has to be
//! installed by the time the device first evaluates. The models a run needs
//! are not only its sealed runtimes: a parameter that reshapes a module
//! compiles that instance anew, and an inserted connect module brings its
//! own. So the host builds the run's circuit once first and keeps every model
//! that build's devices compiled ([`collect_browser_models`]), then runs with
//! the models it installed declared ([`with_browser_run_models`]).
//!
//! The installed modules live in the worker script's registry, which nothing
//! here can read, so the declaration is the host's word for what it
//! installed. A device whose model is not among them (a structural parameter
//! a sweep changes point by point, say) is refused when it is built, naming
//! its module, instead of failing at its first evaluation.

use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

use super::WasmJitModelArtifact;

thread_local! {
    static COLLECTED: RefCell<Option<Vec<Arc<WasmJitModelArtifact>>>> =
        const { RefCell::new(None) };
    static DECLARED: RefCell<Option<HashSet<String>>> = const { RefCell::new(None) };
}

/// Run `build`, and give back every model a device built in it compiled or
/// reused, once each, in the order first asked for.
pub fn collect_browser_models<T>(build: impl FnOnce() -> T) -> (T, Vec<Arc<WasmJitModelArtifact>>) {
    let outer = COLLECTED.with(|slot| slot.replace(Some(Vec::new())));
    let value = build();
    let collected = COLLECTED.with(|slot| slot.replace(outer));
    (value, collected.unwrap_or_default())
}

/// Run `run` with `installed` as the models the browser installed for it, by
/// cache key: a device whose model is not among them is refused when built.
pub fn with_browser_run_models<T>(installed: HashSet<String>, run: impl FnOnce() -> T) -> T {
    let outer = DECLARED.with(|slot| slot.replace(Some(installed)));
    let value = run();
    DECLARED.with(|slot| slot.replace(outer));
    value
}

/// Take `artifact` for a device of `module` being built: keep it while a
/// build is collected, and refuse it while a run that declared its models is
/// running and this is not one of them.
pub(crate) fn admit(artifact: &Arc<WasmJitModelArtifact>, module: &str) -> Result<(), String> {
    COLLECTED.with(|slot| {
        if let Some(collected) = slot.borrow_mut().as_mut()
            && !collected
                .iter()
                .any(|known| known.cache_key() == artifact.cache_key())
        {
            collected.push(Arc::clone(artifact));
        }
    });
    DECLARED.with(|slot| match slot.borrow().as_ref() {
        Some(installed) if !installed.contains(artifact.cache_key()) => Err(format!(
            "module '{module}' compiles here to a browser JIT model (key {}) the run did not \
             install before it started; the browser installs a run's models only from the \
             build it makes before starting, and this compile was not in that build",
            artifact.cache_key()
        )),
        _ => Ok(()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CompilerOptions, VerilogACompiler};

    fn artifact(resistance: f64) -> Arc<WasmJitModelArtifact> {
        let source = format!(
            "`include \"disciplines.vams\"\nmodule run_model(p, n);\n  inout p, n;\n  \
             electrical p, n;\n  analog I(p, n) <+ V(p, n) / {resistance:?};\nendmodule\n"
        );
        let report = VerilogACompiler::new(CompilerOptions::default())
            .compile_runtime(&source, Some("run_model"))
            .expect("compile");
        Arc::new(
            super::super::compile_model_value_module(&report.model, &report.canonical_ir)
                .expect("JIT model"),
        )
    }

    #[test]
    fn a_build_keeps_each_model_once_and_a_run_refuses_one_it_did_not_install() {
        let (one, two) = (artifact(1.0e3), artifact(2.0e3));
        assert_ne!(one.cache_key(), two.cache_key());
        let ((), collected) = collect_browser_models(|| {
            admit(&one, "run_model").unwrap();
            admit(&two, "run_model").unwrap();
            admit(&one, "run_model").unwrap();
        });
        let keys: Vec<&str> = collected.iter().map(|model| model.cache_key()).collect();
        assert_eq!(keys, [one.cache_key(), two.cache_key()]);

        let installed = HashSet::from([one.cache_key().to_owned()]);
        with_browser_run_models(installed, || {
            assert!(admit(&one, "run_model").is_ok());
            let refusal = admit(&two, "run_model").unwrap_err();
            assert!(refusal.contains("module 'run_model'"), "{refusal}");
            assert!(refusal.contains(two.cache_key()), "{refusal}");
        });
        // Outside a declared run every model is taken, and nothing collects.
        assert!(admit(&two, "run_model").is_ok());
        let ((), none) = collect_browser_models(|| {});
        assert!(none.is_empty());
    }
}
