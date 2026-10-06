//! Retained compiler findings shared by a run's resolved and parallel engines.

use super::Engine;
use rspice_veriloga::SourceCompileDiagnostic;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};

pub(super) type CompilerDiagnosticHandler = Arc<dyn Fn(&SourceCompileDiagnostic) + Send + Sync>;

#[derive(Default)]
pub(super) struct CompilerDiagnosticState {
    diagnostics: Mutex<RetainedDiagnostics>,
}

#[derive(Default)]
struct RetainedDiagnostics {
    ordered: Vec<Arc<SourceCompileDiagnostic>>,
    seen: HashSet<Arc<SourceCompileDiagnostic>>,
}

impl Engine {
    /// Observe each distinct compiler finding once for this engine and engines
    /// derived from it. The handler runs outside the collection lock and may
    /// be called by parallel workers; it must not panic. Existing findings are
    /// available through [`Self::compiler_diagnostics`].
    pub fn with_compiler_diagnostic_handler(
        mut self,
        handler: impl Fn(&SourceCompileDiagnostic) + Send + Sync + 'static,
    ) -> Self {
        self.compiler_diagnostic_handler = Some(Arc::new(handler));
        self
    }

    /// Compiler findings retained across builds, cache hits and derived engines.
    pub fn compiler_diagnostics(&self) -> Vec<SourceCompileDiagnostic> {
        let retained = self
            .compiler_diagnostics
            .diagnostics
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        retained
            .ordered
            .iter()
            .map(|diagnostic| diagnostic.as_ref().clone())
            .collect()
    }

    /// Clear retained compiler findings, allowing a new run to report them again.
    pub fn clear_compiler_diagnostics(&self) {
        let mut retained = self
            .compiler_diagnostics
            .diagnostics
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        *retained = RetainedDiagnostics::default();
    }

    pub(super) fn record_compiler_diagnostics(&self, diagnostics: &[SourceCompileDiagnostic]) {
        let fresh = {
            let mut retained = self
                .compiler_diagnostics
                .diagnostics
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let mut fresh = Vec::new();
            for diagnostic in diagnostics {
                if retained.seen.contains(diagnostic) {
                    continue;
                }
                let diagnostic = Arc::new(diagnostic.clone());
                retained.seen.insert(Arc::clone(&diagnostic));
                retained.ordered.push(Arc::clone(&diagnostic));
                fresh.push(diagnostic);
            }
            fresh
        };
        if let Some(handler) = &self.compiler_diagnostic_handler {
            for diagnostic in fresh {
                handler(&diagnostic);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn derived_engines_share_findings_and_notify_once_outside_the_lock() {
        let notifications = Arc::new(AtomicUsize::new(0));
        let engine: Arc<Engine> = Arc::new_cyclic(|weak: &std::sync::Weak<Engine>| {
            let observer = weak.clone();
            let notifications = Arc::clone(&notifications);
            Engine::default().with_compiler_diagnostic_handler(move |diagnostic| {
                // Calling the getter from the handler must not deadlock.
                assert!(
                    observer
                        .upgrade()
                        .unwrap()
                        .compiler_diagnostics()
                        .contains(diagnostic)
                );
                notifications.fetch_add(1, Ordering::Relaxed);
            })
        });
        let diagnostic = SourceCompileDiagnostic {
            severity: rspice_veriloga::CompileDiagnosticSeverity::Warning,
            phase: rspice_veriloga::CompileDiagnosticPhase::Semantic,
            code: "VA-SEM-NO-EFFECT-SYSTEM-TASK".into(),
            message: "task is discarded".into(),
            path: Some("model.va".into()),
            byte_start: Some(0),
            byte_end: Some(1),
            line: Some(1),
            column: Some(1),
        };
        let netlist =
            crate::Netlist::parse("diagnostic session\nV1 p 0 1\nR1 p 0 1\n.end\n").unwrap();
        let derived = [
            engine.resolved_for_netlist(&netlist),
            engine
                .try_resolved_with_config(crate::SimulationConfig::default())
                .unwrap(),
        ];
        std::thread::scope(|scope| {
            for derived in &derived {
                let diagnostic = &diagnostic;
                scope.spawn(move || {
                    for _ in 0..8 {
                        derived.record_compiler_diagnostics(std::slice::from_ref(diagnostic));
                    }
                });
            }
        });
        assert_eq!(engine.compiler_diagnostics(), vec![diagnostic.clone()]);
        assert_eq!(notifications.load(Ordering::Relaxed), 1);
        engine.clear_compiler_diagnostics();
        assert!(derived[0].compiler_diagnostics().is_empty());
        derived[1].record_compiler_diagnostics(std::slice::from_ref(&diagnostic));
        assert_eq!(notifications.load(Ordering::Relaxed), 2);
    }
}
