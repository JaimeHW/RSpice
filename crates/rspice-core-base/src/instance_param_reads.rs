//! Which instance parameters a device's build asked for.
//!
//! A native model's accepted instance parameters are the names its build
//! reads. Every reader notes the names it asks for here, so the warning about
//! an unknown instance parameter is the complement of what the model's own
//! code reads and cannot drift from it.

use crate::Value;
use std::cell::RefCell;

struct Watched {
    name: String,
    read: bool,
}

thread_local! {
    static WATCHED: RefCell<Option<Vec<Watched>>> = const { RefCell::new(None) };
}

/// A reader of a device's instance parameters asked for `name`.
#[doc(hidden)]
pub fn note_instance_param_read(name: &str) {
    WATCHED.with(|watched| {
        if let Some(entries) = watched.borrow_mut().as_mut() {
            for entry in entries {
                entry.read |= entry.name.eq_ignore_ascii_case(name);
            }
        }
    });
}

/// Watches one device's instance parameters while its build reads them. The
/// watch ends when this is dropped, and a watch started inside another puts
/// the outer one back.
#[must_use = "the watch ends when this is dropped"]
#[doc(hidden)]
pub struct InstanceParamReads {
    enclosing: Option<Vec<Watched>>,
}

impl InstanceParamReads {
    /// Start watching `params` on this thread.
    #[doc(hidden)]
    pub fn watch(params: &[(String, Value)]) -> Self {
        let entries = params
            .iter()
            .map(|(name, _)| Watched {
                name: name.clone(),
                read: false,
            })
            .collect();
        Self {
            enclosing: WATCHED.with(|watched| watched.borrow_mut().replace(entries)),
        }
    }

    /// The watched parameters nothing asked for, in instance-line order.
    #[doc(hidden)]
    pub fn unread(self) -> Vec<String> {
        WATCHED.with(|watched| {
            watched.borrow().as_ref().map_or_else(Vec::new, |entries| {
                entries
                    .iter()
                    .filter(|entry| !entry.read)
                    .map(|entry| entry.name.clone())
                    .collect()
            })
        })
    }
}

impl Drop for InstanceParamReads {
    fn drop(&mut self) {
        WATCHED.with(|watched| *watched.borrow_mut() = self.enclosing.take());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn params(names: &[&str]) -> Vec<(String, Value)> {
        names.iter().map(|name| ((*name).to_owned(), 1.0)).collect()
    }

    #[test]
    fn a_parameter_nothing_asked_for_is_unread_whatever_its_case() {
        let reads = InstanceParamReads::watch(&params(&["w", "L", "ww"]));
        note_instance_param_read("W");
        note_instance_param_read("l");
        note_instance_param_read("NRD");
        assert_eq!(reads.unread(), ["ww"]);
    }

    #[test]
    fn a_watch_inside_another_puts_the_outer_one_back() {
        let outer = InstanceParamReads::watch(&params(&["w", "foo"]));
        {
            let inner = InstanceParamReads::watch(&params(&["bar"]));
            note_instance_param_read("w");
            assert_eq!(inner.unread(), ["bar"]);
        }
        assert_eq!(outer.unread(), ["w", "foo"]);
    }
}
