//! The deck's indexes, built away from the editor's thread.
//!
//! An index costs the whole deck, so it is built once per text on a thread
//! of its own, the way the Verilog-A live check runs: while the writer types,
//! the frame asks for the index of the text it shows, gets the last one
//! built while the build of that text runs, and the finished build wakes the
//! next frame. Text that was not typed, and an action that must answer about
//! the text as it stands (go to definition), wait for the build instead. The
//! browser build has no threads and builds in place, once per text.

use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::{Mutex, mpsc};

/// One index kept for the editor: the last one built, with the key of the
/// text it was built from, and the build of a newer text while that runs.
pub(crate) struct BackgroundIndex<K, T> {
    built: Option<(K, Arc<T>)>,
    #[cfg(not(target_arch = "wasm32"))]
    running: Option<RunningBuild<K, T>>,
}

#[cfg(not(target_arch = "wasm32"))]
struct RunningBuild<K, T> {
    key: K,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl<K: Clone, T> Clone for RunningBuild<K, T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            receiver: Arc::clone(&self.receiver),
        }
    }
}

impl<K: Clone, T> Clone for BackgroundIndex<K, T> {
    fn clone(&self) -> Self {
        Self {
            built: self.built.clone(),
            #[cfg(not(target_arch = "wasm32"))]
            running: self.running.clone(),
        }
    }
}

impl<K, T> Default for BackgroundIndex<K, T> {
    fn default() -> Self {
        Self {
            built: None,
            #[cfg(not(target_arch = "wasm32"))]
            running: None,
        }
    }
}

impl<K, T> std::fmt::Debug for BackgroundIndex<K, T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        #[cfg(not(target_arch = "wasm32"))]
        let running = self.running.is_some();
        #[cfg(target_arch = "wasm32")]
        let running = false;
        formatter
            .debug_struct("BackgroundIndex")
            .field("built", &self.built.is_some())
            .field("running", &running)
            .finish()
    }
}

impl<K: Send + 'static, T: Send + Sync + 'static> BackgroundIndex<K, T> {
    /// The last index built, whatever text it was built from.
    pub(crate) fn last_built(&self) -> Option<Arc<T>> {
        self.built.as_ref().map(|(_, index)| Arc::clone(index))
    }

    /// Take a build that has finished.
    fn take_finished(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(running) = self.running.take() {
            let received = running
                .receiver
                .lock()
                .map_or(Err(mpsc::TryRecvError::Disconnected), |receiver| {
                    receiver.try_recv()
                });
            match received {
                Ok(index) => self.built = Some((running.key, Arc::new(index))),
                Err(mpsc::TryRecvError::Empty) => self.running = Some(running),
                Err(mpsc::TryRecvError::Disconnected) => {}
            }
        }
    }

    /// Whether the text `is_current` accepts is neither built nor building.
    /// A build of a text that has moved on is dropped: its answer is of
    /// another text, and its thread finishes into a closed channel.
    pub(crate) fn wants(&mut self, is_current: impl Fn(&K) -> bool) -> bool {
        self.take_finished();
        if self.built.as_ref().is_some_and(|(key, _)| is_current(key)) {
            return false;
        }
        self.drop_build_unless(&is_current);
        !self.is_building()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn drop_build_unless(&mut self, is_current: &impl Fn(&K) -> bool) {
        if self
            .running
            .as_ref()
            .is_some_and(|running| !is_current(&running.key))
        {
            self.running = None;
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn drop_build_unless(&mut self, _is_current: &impl Fn(&K) -> bool) {}

    #[cfg(not(target_arch = "wasm32"))]
    const fn is_building(&self) -> bool {
        self.running.is_some()
    }

    #[cfg(target_arch = "wasm32")]
    const fn is_building(&self) -> bool {
        false
    }

    /// Build the index of the text `key` names; `wake` is called when it is
    /// done.
    pub(crate) fn start(
        &mut self,
        key: K,
        build: impl FnOnce() -> T + Send + 'static,
        wake: impl FnOnce() + Send + 'static,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (sender, receiver) = mpsc::channel();
            let spawned = std::thread::Builder::new()
                .name("netlist-index".to_owned())
                .spawn(move || {
                    if sender.send(build()).is_ok() {
                        wake();
                    }
                });
            if spawned.is_ok() {
                self.running = Some(RunningBuild {
                    key,
                    receiver: Arc::new(Mutex::new(receiver)),
                });
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = wake;
            self.built = Some((key, Arc::new(build())));
        }
    }

    /// The index of the text `is_current` accepts, if it is built or being
    /// built, waiting for a running build to finish.
    pub(crate) fn wait_for(&mut self, is_current: impl Fn(&K) -> bool) -> Option<Arc<T>> {
        self.take_finished();
        if let Some((key, index)) = &self.built
            && is_current(key)
        {
            return Some(Arc::clone(index));
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(running) = self.running.take_if(|running| is_current(&running.key)) {
            let received = running
                .receiver
                .lock()
                .ok()
                .and_then(|receiver| receiver.recv().ok());
            if let Some(index) = received {
                let index = Arc::new(index);
                self.built = Some((running.key, Arc::clone(&index)));
                return Some(index);
            }
        }
        None
    }

    /// Keep `index`, built here, as the index of the text `key` names.
    pub(crate) fn store(&mut self, key: K, index: T) -> Arc<T> {
        let index = Arc::new(index);
        self.built = Some((key, Arc::clone(&index)));
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_gets_the_last_index_while_the_next_one_builds_and_an_action_waits_for_it() {
        let mut index = BackgroundIndex::<&'static str, usize>::default();
        let (release, gate) = std::sync::mpsc::channel::<()>();
        assert!(index.wants(|key| *key == "one"));
        index.start(
            "one",
            move || {
                gate.recv().ok();
                1
            },
            || {},
        );
        assert!(!index.wants(|key| *key == "one"), "one build per text");
        assert_eq!(
            index.last_built(),
            None,
            "nothing stands in until a build finishes"
        );
        release.send(()).expect("the build waits for this");
        assert_eq!(index.wait_for(|key| *key == "one").as_deref(), Some(&1));

        assert!(index.wants(|key| *key == "two"));
        index.start("two", || 2, || {});
        assert_eq!(
            index.wait_for(|key| *key == "three"),
            None,
            "a build of another text answers nothing about this one"
        );
        assert_eq!(index.wait_for(|key| *key == "two").as_deref(), Some(&2));
    }
}
