//! Keeping the session for the next start.
//!
//! A browser keeps the session in its local storage, which holds a few
//! megabytes for the whole site, and the waveforms of one long run are more
//! than that. A refused write reaches only eframe's log, so on the browser the
//! stored session is read back. A refused session is offered again without
//! its runs: a reload then brings back the project, its documents and the
//! layout instead of whatever session the store last accepted, and the reader
//! is told once.

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::lifecycle::session::SessionKept;

use super::serialization::WithoutRuns;

/// What the store did with one encoding of the session.
enum Offer {
    Kept,
    Refused,
    NotEncoded,
}

impl AppState {
    /// Save the session under eframe's application key.
    pub(crate) fn store_session(&mut self, storage: &mut dyn eframe::Storage) {
        // Only the browser's store refuses without saying so. A desktop store
        // holds what it is given until it writes its file.
        self.store_session_checked(storage, cfg!(target_arch = "wasm32"));
    }

    fn store_session_checked(&mut self, storage: &mut dyn eframe::Storage, read_back: bool) {
        let history = self.simulation.retained.runs.revision();
        if self.ui.session_kept != SessionKept::WithoutRuns(history.clone()) {
            match offer(storage, ron::ser::to_string(&*self), read_back) {
                Offer::Kept => {
                    self.settle(SessionKept::Whole);
                    return;
                }
                Offer::Refused => {}
                Offer::NotEncoded => return,
            }
        }
        let kept = match offer(storage, ron::ser::to_string(&WithoutRuns(self)), read_back) {
            Offer::Kept => SessionKept::WithoutRuns(history),
            Offer::Refused => SessionKept::Nothing,
            Offer::NotEncoded => return,
        };
        self.settle(kept);
    }

    /// Remember what this save kept, and say so when that changes.
    fn settle(&mut self, kept: SessionKept) {
        let before = std::mem::replace(&mut self.ui.session_kept, kept);
        match (&before, &self.ui.session_kept) {
            (SessionKept::Whole, SessionKept::Whole)
            | (SessionKept::WithoutRuns(_), SessionKept::WithoutRuns(_))
            | (SessionKept::Nothing, SessionKept::Nothing) => {}
            (_, SessionKept::Whole) => {
                self.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    "This browser keeps the whole session again, run results included.",
                );
            }
            (_, SessionKept::WithoutRuns(_)) => self.report_warning(
                "Run results are not kept for the next session",
                "They are more than this browser's storage holds, so a reload brings back the \
                 project without them. Save the project to keep them.",
            ),
            (_, SessionKept::Nothing) => self.report_failure(
                "The session could not be kept",
                "This browser's storage refused it, so a reload returns to the last session it \
                 kept.",
            ),
        }
    }
}

fn offer(
    storage: &mut dyn eframe::Storage,
    encoded: ron::Result<String>,
    read_back: bool,
) -> Offer {
    let text = match encoded {
        Ok(text) => text,
        Err(error) => {
            // What eframe's own encoder does: the stored session stays.
            log::error!("The session could not be encoded: {error}");
            return Offer::NotEncoded;
        }
    };
    let expected = read_back.then(|| fingerprint(&text));
    storage.set_string(eframe::APP_KEY, text);
    match expected {
        Some(expected)
            if storage
                .get_string(eframe::APP_KEY)
                .is_none_or(|stored| fingerprint(&stored) != expected) =>
        {
            Offer::Refused
        }
        Some(_) | None => Offer::Kept,
    }
}

/// Compares a stored session with the one offered without keeping a second
/// copy of a session that may be hundreds of megabytes.
fn fingerprint(text: &str) -> (usize, u64) {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    (text.len(), hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A store that, like a browser's local storage, keeps its old value when
    /// a new one is over its limit.
    #[derive(Default)]
    struct LimitedStorage {
        limit: usize,
        value: Option<String>,
        writes: usize,
    }

    impl eframe::Storage for LimitedStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            (key == eframe::APP_KEY)
                .then(|| self.value.clone())
                .flatten()
        }

        fn set_string(&mut self, key: &str, value: String) {
            assert_eq!(key, eframe::APP_KEY);
            self.writes += 1;
            if value.len() <= self.limit {
                self.value = Some(value);
            }
        }

        fn remove_string(&mut self, _key: &str) {
            self.value = None;
        }

        fn flush(&mut self) {}
    }

    fn session_with_a_long_run() -> AppState {
        let mut state = AppState::default();
        let time: Vec<f64> = (0..20_000_u32).map(|step| f64::from(step) * 1e-9).collect();
        let volts: Vec<f64> = time.iter().map(|t| (t * 1e7).sin()).collect();
        let mut run = crate::state::SimulationRun::new(7);
        run.add_analysis(
            crate::state::AnalysisResult::new(1, crate::state::AnalysisType::Transient, "TRAN")
                .with_waveforms(vec![crate::state::WaveformData::new(
                    "V(out)", time, volts, "#00aaff",
                )]),
        );
        run.restore_provenance(crate::state::SimulationRunProvenance::LegacyUnattributed)
            .expect("legacy fixture provenance seals");
        state.simulation.retained.runs = vec![run].into();
        state.simulation.retained.next_run_id = 7;
        state
    }

    /// A store that holds the session without its runs, and not with them.
    fn storage_between(state: &AppState) -> LimitedStorage {
        let whole = ron::ser::to_string(state).expect("session encodes").len();
        let lean = ron::ser::to_string(&WithoutRuns(state))
            .expect("session encodes")
            .len();
        assert!(lean * 2 < whole, "the run must dominate the session");
        LimitedStorage {
            limit: (lean + whole) / 2,
            ..LimitedStorage::default()
        }
    }

    fn console(state: &AppState) -> Vec<(LogSeverity, String)> {
        state
            .log_buffer
            .entries()
            .map(|entry| (entry.severity, entry.message.clone()))
            .collect()
    }

    #[test]
    fn a_session_the_store_refuses_is_kept_without_its_runs_and_said_once() {
        let mut state = session_with_a_long_run();
        let mut storage = storage_between(&state);

        state.store_session_checked(&mut storage, true);

        let restored = AppState::restore_eframe_session(&storage).expect("a stored session");
        assert!(restored.simulation.retained.runs.is_empty());
        assert_eq!(
            restored.simulation.retained.next_run_id, 7,
            "the next run is not numbered as the one left out"
        );
        assert!(matches!(state.ui.session_kept, SessionKept::WithoutRuns(_)));
        let said = console(&state);
        assert_eq!(said.len(), 1, "{said:?}");
        assert_eq!(said[0].0, LogSeverity::Warning);
        assert!(
            said[0].1.contains("Save the project to keep them"),
            "{said:?}"
        );

        // The same runs are not offered again, and the refusal is not said again.
        storage.writes = 0;
        state.store_session_checked(&mut storage, true);
        assert_eq!(storage.writes, 1);
        assert_eq!(console(&state).len(), 1);
    }

    #[test]
    fn the_whole_session_is_kept_again_once_the_runs_change() {
        let mut state = session_with_a_long_run();
        let mut storage = storage_between(&state);
        state.store_session_checked(&mut storage, true);

        state.simulation.clear_runs();
        state.store_session_checked(&mut storage, true);

        assert_eq!(state.ui.session_kept, SessionKept::Whole);
        let said = console(&state);
        assert_eq!(said.len(), 2, "{said:?}");
        assert_eq!(said[1].0, LogSeverity::Info);
    }

    #[test]
    fn a_store_that_refuses_even_the_lean_session_is_reported_once() {
        let mut state = session_with_a_long_run();
        let mut storage = LimitedStorage::default();

        state.store_session_checked(&mut storage, true);
        state.store_session_checked(&mut storage, true);

        assert_eq!(state.ui.session_kept, SessionKept::Nothing);
        assert!(storage.value.is_none());
        let said = console(&state);
        assert_eq!(said.len(), 1, "{said:?}");
        assert_eq!(said[0].0, LogSeverity::Error);
    }

    #[test]
    fn a_store_that_is_not_read_back_keeps_the_whole_session() {
        let mut state = session_with_a_long_run();
        let mut storage = LimitedStorage {
            limit: usize::MAX,
            ..LimitedStorage::default()
        };

        state.store_session_checked(&mut storage, false);

        assert_eq!(storage.writes, 1);
        let restored = AppState::restore_eframe_session(&storage).expect("a stored session");
        assert_eq!(
            restored.simulation.retained.run_count(),
            1,
            "{:?}",
            console(&restored)
        );
        assert_eq!(state.ui.session_kept, SessionKept::Whole);
        assert!(console(&state).is_empty());
    }
}
