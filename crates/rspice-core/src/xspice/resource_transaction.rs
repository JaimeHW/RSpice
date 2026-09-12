//! Undo images for shared resources advanced during circuit acceptance.

use super::{CmError, CmResult};
use std::any::Any;
use std::collections::HashSet;
use std::fmt;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::{Arc, Mutex};

/// A reversible, shared host resource used by an XSPICE model.
///
/// Capture must be observational and include every mutable fact needed to undo
/// subsequent calls, including runtime time, queues and random state. Restoring
/// an image must undo external effects as well as memory changes. These images
/// are in-process transactions, not a portable restart format. Callbacks must
/// not re-enter model evaluation or context resource access.
///
/// Register through `CmContext::set_transactional_resource` and obtain every
/// access through `CmContext::transactional_resource`. Do not retain a resource
/// handle across evaluations or mutate it through an independently held alias.
pub(crate) trait TransactionalContextResource: Any + Send + Sync {
    fn capture_transaction_state(&self) -> CmResult<Vec<u8>>;
    fn restore_transaction_state(&self, state: &[u8]) -> CmResult<()>;
}

pub(crate) struct ResourceEntry {
    resource: Arc<dyn TransactionalContextResource>,
    poison: Mutex<Option<String>>,
}

impl ResourceEntry {
    pub(crate) fn new(resource: Arc<dyn TransactionalContextResource>) -> Self {
        Self {
            resource,
            poison: Mutex::new(None),
        }
    }

    pub(crate) fn check_healthy(&self) -> CmResult<()> {
        match &*self
            .poison
            .lock()
            .unwrap_or_else(|error| error.into_inner())
        {
            Some(reason) => Err(CmError::EvaluationError(format!(
                "resource is unusable after failed rollback: {reason}"
            ))),
            None => Ok(()),
        }
    }

    pub(crate) fn poison(&self, reason: String) {
        *self
            .poison
            .lock()
            .unwrap_or_else(|error| error.into_inner()) = Some(reason);
    }
}

struct Undo {
    owner: usize,
    key: String,
    entry: Arc<ResourceEntry>,
    state: Vec<u8>,
}

#[derive(Default)]
struct Journal {
    closed: bool,
    seen: HashSet<usize>,
    undo: Vec<Undo>,
}

/// Cloned evaluation scopes all enlist in the same acceptance transaction.
/// The caller must explicitly commit or roll back before discarding it.
#[derive(Clone, Default)]
#[must_use]
pub(crate) struct ResourceTransaction(Arc<Mutex<Journal>>);

impl fmt::Debug for ResourceTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let journal = self.0.lock().unwrap_or_else(|error| error.into_inner());
        f.debug_struct("ResourceTransaction")
            .field("closed", &journal.closed)
            .field("resources", &journal.undo.len())
            .finish()
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ResourceTransactionScope {
    pub(crate) transaction: ResourceTransaction,
    pub(crate) owner: usize,
}

pub(crate) struct ResourceRollbackFailure {
    pub(crate) owner: usize,
    pub(crate) key: String,
    pub(crate) detail: String,
}

fn provider_call<T>(operation: &str, call: impl FnOnce() -> CmResult<T>) -> CmResult<T> {
    catch_unwind(AssertUnwindSafe(call)).unwrap_or_else(|_| {
        Err(CmError::EvaluationError(format!(
            "resource provider panicked during {operation}"
        )))
    })
}

impl ResourceTransaction {
    pub(crate) fn enlist(
        &self,
        owner: usize,
        key: &str,
        entry: Arc<ResourceEntry>,
    ) -> CmResult<()> {
        entry.check_healthy()?;
        let mut journal = self.0.lock().unwrap_or_else(|error| error.into_inner());
        if journal.closed {
            return Err(CmError::EvaluationError(
                "resource transaction is already closed".into(),
            ));
        }
        let identity = Arc::as_ptr(&entry) as usize;
        if journal.seen.contains(&identity) {
            return Ok(());
        }
        let state = provider_call("capture", || entry.resource.capture_transaction_state())?;
        journal.seen.insert(identity);
        journal.undo.push(Undo {
            owner,
            key: key.into(),
            entry,
            state,
        });
        Ok(())
    }

    pub(crate) fn commit(&self) {
        let mut journal = self.0.lock().unwrap_or_else(|error| error.into_inner());
        journal.closed = true;
        journal.undo.clear();
        journal.seen.clear();
    }

    pub(crate) fn rollback(&self) -> Vec<ResourceRollbackFailure> {
        let undo = {
            let mut journal = self.0.lock().unwrap_or_else(|error| error.into_inner());
            journal.closed = true;
            journal.seen.clear();
            std::mem::take(&mut journal.undo)
        };
        let mut failures = Vec::new();
        for undo in undo.into_iter().rev() {
            if let Err(error) = provider_call("restore", || {
                undo.entry.resource.restore_transaction_state(&undo.state)
            }) {
                let detail = error.to_string();
                undo.entry.poison(detail.clone());
                failures.push(ResourceRollbackFailure {
                    owner: undo.owner,
                    key: undo.key,
                    detail,
                });
            }
        }
        failures
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::CmContext;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct Provider {
        id: usize,
        value: AtomicUsize,
        restores: Arc<Mutex<Vec<usize>>>,
    }

    impl TransactionalContextResource for Provider {
        fn capture_transaction_state(&self) -> CmResult<Vec<u8>> {
            Ok(self.value.load(Ordering::Relaxed).to_le_bytes().to_vec())
        }
        fn restore_transaction_state(&self, state: &[u8]) -> CmResult<()> {
            self.restores.lock().unwrap().push(self.id);
            match self.id {
                2 => return Err(CmError::EvaluationError("injected restore failure".into())),
                3 => panic!("injected provider panic"),
                _ => {}
            }
            self.value.store(
                usize::from_le_bytes(state.try_into().unwrap()),
                Ordering::Relaxed,
            );
            Ok(())
        }
    }

    #[test]
    fn resource_transaction_restores_all_in_reverse_and_poison_survives_context_clones() {
        let restores = Arc::new(Mutex::new(Vec::new()));
        let mut ctx = CmContext::new();
        for id in 1..=3 {
            ctx.set_transactional_resource(
                id.to_string(),
                Arc::new(Provider {
                    id,
                    value: AtomicUsize::new(id),
                    restores: restores.clone(),
                }),
            );
        }
        let accepted = ctx.clone();
        let journal = ResourceTransaction::default();
        ctx.set_resource_transaction(Some(ResourceTransactionScope {
            transaction: journal.clone(),
            owner: 7,
        }));
        for id in 1..=3 {
            let key = id.to_string();
            for _ in 0..2 {
                ctx.transactional_resource::<Provider>(&key)
                    .unwrap()
                    .unwrap()
                    .value
                    .fetch_add(10, Ordering::Relaxed);
            }
            assert!(
                ctx.resource::<Provider>(&key).is_none(),
                "ordinary access cannot bypass the journal"
            );
        }
        let failures = journal.rollback();
        assert_eq!(*restores.lock().unwrap(), [3, 2, 1]);
        assert_eq!(failures.len(), 2);
        assert!(failures.iter().all(|failure| failure.owner == 7));
        assert_eq!(failures[0].key, "3");
        assert!(failures[0].detail.contains("panicked"));
        assert_eq!(
            accepted
                .transactional_resource::<Provider>("1")
                .unwrap()
                .unwrap()
                .value
                .load(Ordering::Relaxed),
            1
        );
        for key in ["2", "3"] {
            let error = accepted
                .transactional_resource::<Provider>(key)
                .err()
                .unwrap();
            assert!(error.to_string().contains("unusable after failed rollback"));
        }
        assert!(
            ctx.transactional_resource::<Provider>("1").is_err(),
            "a closed scope cannot be reused"
        );
    }
}
