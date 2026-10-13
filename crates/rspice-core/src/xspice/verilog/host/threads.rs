//! `disable` across threads (IEEE 1364-2005 section 10.3, design P2 §4.1).
//!
//! Disabling a named block or a task ends every execution of it, in every
//! thread. Each thread inside the target goes on from the state its resume
//! state unwinds to; a thread forked inside the target ends, with every
//! thread it forked. Nonblocking updates already scheduled stay scheduled.

use super::*;
use rspice_veriloga::canonical_ir::digital_eval::DigitalUnwind;

impl DigitalHost {
    /// Carry out a `disable` of `target` that the thread in slot `disabler`
    /// reached; that thread's state is already in its slot. Returns whether
    /// the disabling thread lives on, to run on at once.
    pub(super) fn disable(
        &mut self,
        disabler: usize,
        target: DigitalDisableTarget,
        tick: u64,
    ) -> Result<bool, DigitalRunError> {
        let mut unwound = BTreeMap::new();
        for (index, slot) in self.slots.iter().enumerate() {
            if slot.status == ProcessStatus::Finished {
                continue;
            }
            let Some(state) = slot.resume.as_ref() else {
                continue;
            };
            match state.unwind(&self.plan, target, index == disabler) {
                DigitalUnwind::Untouched => {}
                DigitalUnwind::Resume(state) => {
                    unwound.insert(index, Some(state));
                }
                DigitalUnwind::End => {
                    unwound.insert(index, None);
                }
            }
        }
        // A thread forked by a thread that leaves the target was forked
        // inside it, so it ends, and so does everything it forked.
        let ended: BTreeSet<usize> = (0..self.slots.len())
            .filter(|index| {
                let mut parent = self.slots[*index].parent;
                while let Some(forker) = parent {
                    if unwound.contains_key(&forker) {
                        return true;
                    }
                    parent = self.slots[forker].parent;
                }
                false
            })
            .collect();
        let mut alive = true;
        for index in &ended {
            self.cancel_wait(*index);
            self.end_thread(*index);
            alive &= *index != disabler;
        }
        for (index, state) in unwound {
            if ended.contains(&index) {
                continue;
            }
            self.cancel_wait(index);
            match state {
                Some(state) => {
                    let slot = &mut self.slots[index];
                    slot.resume = Some(state);
                    slot.children = 0;
                    slot.remaining_events = DigitalEventCount::one();
                    if index != disabler {
                        self.queue(index, tick)?;
                    }
                }
                None => {
                    self.end_child(index, tick)?;
                    alive &= index != disabler;
                }
            }
        }
        Ok(alive)
    }

    /// Withdraw whatever a thread waits on, leaving it queued in name only.
    fn cancel_wait(&mut self, index: usize) {
        match &self.slots[index].status {
            ProcessStatus::AwaitingEvent(_) => self.unsubscribe(index),
            ProcessStatus::AwaitingExpression(token) => {
                let token = *token;
                self.store.cancel_expression_wait(token);
                self.expression_processes.remove(&token);
            }
            ProcessStatus::AwaitingAnalog(_) => {
                self.analog_waiters.retain(|waiter| *waiter != index)
            }
            ProcessStatus::Inactive => self.inactive.retain(|waiter| *waiter != index),
            ProcessStatus::Queued => {
                let target = self.targets[index];
                self.scheduler.cancel_target(target);
                if let Some(ready) = self.analog_ready.as_mut() {
                    ready.retain(|ready| *ready != target);
                }
            }
            ProcessStatus::AwaitingJoin | ProcessStatus::Finished => {}
        }
        self.slots[index].status = ProcessStatus::Queued;
    }

    /// End a forked thread whose parent ends or leaves with it, and free its
    /// slot.
    fn end_thread(&mut self, index: usize) {
        let slot = &mut self.slots[index];
        slot.status = ProcessStatus::Finished;
        slot.resume = None;
        slot.parent = None;
        slot.children = 0;
        self.free_threads.insert(index);
    }
}
