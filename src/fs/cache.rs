/*
MIT License

Copyright (c) 2025 Ritchie Mwewa

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::fs::metadata::Metadata;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

static TRUE_SIZE_CACHE: OnceLock<Mutex<HashMap<(PathBuf, bool), u64>>> = OnceLock::new();
static SIZE_DISPLAY_CACHE: OnceLock<Mutex<HashMap<u64, Arc<str>>>> = OnceLock::new();

#[cfg(all(feature = "magic", not(target_os = "android")))]
static MAGIC_CACHE: OnceLock<Mutex<HashMap<PathBuf, Arc<str>>>> = OnceLock::new();

static NUMBER_DISPLAY_CACHE: OnceLock<Mutex<HashMap<u64, Arc<str>>>> = OnceLock::new();
static DATE_DISPLAY_CACHE: OnceLock<Mutex<HashMap<Option<SystemTime>, Arc<str>>>> = OnceLock::new();
static PERMISSIONS_CACHE: OnceLock<Mutex<HashMap<u32, Arc<str>>>> = OnceLock::new();
static USER_CACHE: OnceLock<Mutex<HashMap<u32, Arc<str>>>> = OnceLock::new();
static GROUP_CACHE: OnceLock<Mutex<HashMap<u32, Arc<str>>>> = OnceLock::new();

/// Thread-safe caching layer for formatted display strings and computed values.
///
/// Each cache method checks a global `OnceLock<Mutex<HashMap>>` before calling
/// the provided formatting/compute closure, ensuring each unique key is only
/// computed once.
pub(crate) struct Cache;

impl Cache {
    /// Loads metadata for a path. Not cached — delegates directly to [`Metadata::load`].
    ///
    /// # Parameters
    /// - `path`: The filesystem path to query.
    /// - `dereference`: If `true`, follows symlinks (stat); otherwise uses lstat.
    ///
    /// # Returns
    /// The loaded [`Metadata`], or an I/O error.
    pub(crate) fn metadata(path: &Path, dereference: bool) -> io::Result<Metadata> {
        Metadata::load(path, dereference)
    }

    /// Returns a cached formatted string for a number, computing it via `format` on a cache miss.
    ///
    /// # Parameters
    /// - `number`: The numeric key to look up or cache.
    /// - `format`: Closure to produce the display string on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly computed display string.
    pub(crate) fn number(number: u64, format: impl Fn(u64) -> Arc<str>) -> Arc<str> {
        let cache = NUMBER_DISPLAY_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &number) {
            return cached;
        }

        let formatted = format(number);
        Self::setter(cache, number, formatted.clone());
        formatted
    }

    /// Returns a cached permission string for a Unix mode, computing it via `format` on a cache miss.
    ///
    /// # Parameters
    /// - `mode`: The raw Unix permission bits.
    /// - `format`: Closure to produce the display string (e.g. `"rwxr-xr-x"`) on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly computed permission string.
    pub(crate) fn permissions(mode: u32, format: impl Fn(u32) -> Arc<str>) -> Arc<str> {
        let cache = PERMISSIONS_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &mode) {
            return cached;
        }

        let formatted = format(mode);
        Self::setter(cache, mode, formatted.clone());
        formatted
    }

    /// Returns a cached human-readable size string, computing it via `format` on a cache miss.
    ///
    /// # Parameters
    /// - `bytes`: The raw byte count.
    /// - `format`: Closure to produce the display string (e.g. `"4.2 KiB"`) on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly computed size string.
    pub(crate) fn size(bytes: u64, format: impl Fn(u64) -> Arc<str>) -> Arc<str> {
        let cache = SIZE_DISPLAY_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &bytes) {
            return cached;
        }

        let formatted = format(bytes);
        Self::setter(cache, bytes, formatted.clone());
        formatted
    }

    /// Returns a cached recursive directory size, computing it via `compute` on a cache miss.
    ///
    /// # Parameters
    /// - `path`: The directory path to compute size for.
    /// - `include_hidden`: Whether hidden files are included (part of the cache key).
    /// - `compute`: Closure to calculate the total size on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly computed total size in bytes.
    pub(crate) fn true_size(
        path: &Path,
        include_hidden: bool,
        compute: impl FnOnce() -> u64,
    ) -> u64 {
        let cache = TRUE_SIZE_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &(path.to_path_buf(), include_hidden)) {
            return cached;
        }

        let size = compute();
        Self::setter(cache, (path.to_path_buf(), include_hidden), size);
        size
    }

    /// Returns a cached username for a UID, resolving it via `lookup` on a cache miss.
    ///
    /// # Parameters
    /// - `uid`: The user ID to resolve.
    /// - `lookup`: Closure to resolve the UID to a username string on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly resolved username.
    pub(crate) fn owner(uid: u32, lookup: impl Fn(u32) -> Arc<str>) -> Arc<str> {
        let cache = USER_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &uid) {
            return cached;
        }

        let formatted = lookup(uid);
        Self::setter(cache, uid, formatted.clone());
        formatted
    }

    /// Returns a cached formatted date string, computing it via `format` on a cache miss.
    ///
    /// # Parameters
    /// - `ts`: The optional timestamp to format. `None` represents an unavailable timestamp.
    /// - `format`: Closure to produce the display string on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly computed date string.
    pub(crate) fn date(
        ts: Option<SystemTime>,
        format: impl Fn(Option<SystemTime>) -> Arc<str>,
    ) -> Arc<str> {
        let cache = DATE_DISPLAY_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &ts) {
            return cached;
        }

        let formatted = format(ts);
        Self::setter(cache, ts, formatted.clone());
        formatted
    }

    /// Returns a cached libmagic file description, computing it via `compute` on a cache miss.
    ///
    /// # Parameters
    /// - `path`: The file path to identify.
    /// - `compute`: Closure to produce the magic description string on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly computed file description.
    #[cfg(all(feature = "magic", not(target_os = "android")))]
    pub(crate) fn magic(path: &PathBuf, compute: impl FnOnce() -> Arc<str>) -> Arc<str> {
        let cache = MAGIC_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, path) {
            return cached;
        }

        let description = compute();
        Self::setter(cache, path.clone(), description.clone());
        description
    }

    /// Returns a cached group name for a GID, resolving it via `lookup` on a cache miss.
    ///
    /// # Parameters
    /// - `gid`: The group ID to resolve.
    /// - `lookup`: Closure to resolve the GID to a group name string on a cache miss.
    ///
    /// # Returns
    /// The cached or freshly resolved group name.
    pub(crate) fn group(gid: u32, lookup: impl Fn(u32) -> Arc<str>) -> Arc<str> {
        let cache = GROUP_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &gid) {
            return cached;
        }

        let formatted = lookup(gid);
        Self::setter(cache, gid, formatted.clone());
        formatted
    }

    /// Attempts to retrieve a cloned value from a locked cache map.
    ///
    /// # Parameters
    /// - `cache`: The mutex-guarded hash map to look up.
    /// - `key`: The key to search for.
    ///
    /// # Returns
    /// `Some(value)` on a cache hit, or `None` on a miss or poisoned lock.
    fn getter<K: Eq + std::hash::Hash, V: Clone>(
        cache: &Mutex<HashMap<K, V>>,
        key: &K,
    ) -> Option<V> {
        if let Ok(map) = cache.lock() {
            map.get(key).cloned()
        } else {
            None
        }
    }

    /// Inserts a key-value pair into a locked cache map. Silently no-ops on a poisoned lock.
    ///
    /// # Parameters
    /// - `cache`: The mutex-guarded hash map to insert into.
    /// - `key`: The cache key.
    /// - `value`: The value to store.
    fn setter<K: Eq + std::hash::Hash, V>(cache: &Mutex<HashMap<K, V>>, key: K, value: V) {
        if let Ok(mut map) = cache.lock() {
            map.insert(key, value);
        }
    }
}
