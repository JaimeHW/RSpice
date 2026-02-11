use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub(crate) fn read_lock<'a, T>(lock: &'a RwLock<T>, context: &str) -> RwLockReadGuard<'a, T> {
    match lock.read() {
        Ok(guard) => guard,
        Err(poisoned) => {
            log::warn!(
                "Recovered poisoned read lock in integration subsystem ({})",
                context
            );
            poisoned.into_inner()
        }
    }
}

pub(crate) fn write_lock<'a, T>(lock: &'a RwLock<T>, context: &str) -> RwLockWriteGuard<'a, T> {
    match lock.write() {
        Ok(guard) => guard,
        Err(poisoned) => {
            log::warn!(
                "Recovered poisoned write lock in integration subsystem ({})",
                context
            );
            poisoned.into_inner()
        }
    }
}
