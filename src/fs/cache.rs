// SPDX-License-Identifier: MIT

//! Memoisation for values that are expensive to compute and repeat across entries.
//!
//! Rendering is single-threaded, so each cache is a plain thread-local map:
//! no locking, no poisoning.

use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

/// Declares a memoised lookup: `name(key) -> value`, computed once per distinct key.
///
/// Expands to a function that takes the key and a closure producing the value on
/// a miss, backed by a private thread-local `HashMap`.
macro_rules! memoise {
    ($(#[$doc:meta])* $name:ident: $key:ty => $value:ty) => {
        $(#[$doc])*
        pub(crate) fn $name(key: $key, compute: impl FnOnce(&$key) -> $value) -> $value {
            thread_local! {
                static CACHE: RefCell<HashMap<$key, $value>> = RefCell::new(HashMap::new());
            }
            Cache::get_or_insert(&CACHE, key, compute)
        }
    };
}

/// Thread-local caching layer for formatted display strings and computed values.
pub(crate) struct Cache;

impl Cache {
    memoise! {
        /// Formatted number (hard links, blocks), keyed by the raw value.
        number: u64 => Arc<str>
    }

    memoise! {
        /// Symbolic/octal/hex permission string, keyed by mode *and* xattr presence:
        /// the rendered string carries the `@` suffix, so the flag is part of the key.
        permissions: (u32, bool) => Arc<str>
    }

    memoise! {
        /// Human-readable size string, keyed by the byte count.
        size: u64 => Arc<str>
    }

    memoise! {
        /// Recursive directory size, keyed by path and whether hidden files count.
        dir_size: (PathBuf, bool) => u64
    }

    memoise! {
        /// Immediate child count of a directory, keyed by path and whether hidden
        /// entries count.
        item_count: (PathBuf, bool) => usize
    }

    memoise! {
        /// Username, keyed by UID.
        owner: u32 => Arc<str>
    }

    memoise! {
        /// Group name, keyed by GID.
        group: u32 => Arc<str>
    }

    memoise! {
        /// Formatted timestamp, keyed by the instant itself.
        date: Option<SystemTime> => Arc<str>
    }

    #[cfg(all(feature = "magic", not(target_os = "android")))]
    memoise! {
        /// libmagic description, keyed by path.
        magic: PathBuf => Arc<str>
    }

    /// Returns the cached value for `key`, computing and storing it on a miss.
    ///
    /// # Parameters
    /// - `cache`: The thread-local map backing this cache.
    /// - `key`: The lookup key.
    /// - `compute`: Produces the value when the key is absent.
    fn get_or_insert<K: Eq + Hash + Clone + 'static, V: Clone + 'static>(
        cache: &'static std::thread::LocalKey<RefCell<HashMap<K, V>>>,
        key: K,
        compute: impl FnOnce(&K) -> V,
    ) -> V {
        if let Some(hit) = cache.with(|map| map.borrow().get(&key).cloned()) {
            return hit;
        }

        let value = compute(&key);
        cache.with(|map| map.borrow_mut().insert(key, value.clone()));
        value
    }
}
