// SPDX-License-Identifier: MIT

//! A process-wide on/off switch resolved once at startup.

use std::sync::atomic::{AtomicBool, Ordering};

/// An output feature that is either on or off for the whole run.
pub struct Toggle(AtomicBool);

impl Toggle {
    /// Creates a toggle in the given starting state.
    ///
    /// # Parameters
    /// - `on`: The state to start in, before `setup` resolves the real one.
    pub const fn new(on: bool) -> Self {
        Self(AtomicBool::new(on))
    }

    /// Turns the feature on or off.
    ///
    /// # Parameters
    /// - `on`: Whether the feature should be enabled.
    pub(crate) fn set(&self, on: bool) {
        self.0.store(on, Ordering::Relaxed);
    }

    /// Reports whether the feature is enabled.
    pub(crate) fn is_enabled(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}
