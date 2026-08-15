// SPDX-License-Identifier: MIT

pub mod cli;
pub mod render;
pub mod fs;

use std::env;

/// Package authors.
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

/// Package version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Package name.
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Package description.
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
