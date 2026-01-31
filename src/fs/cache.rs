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

pub(crate) struct Cache;

impl Cache {
    /// Loads metadata for a path using lstat.
    pub(crate) fn metadata(path: &Path) -> io::Result<Metadata> {
        Metadata::load(path)
    }

    pub(crate) fn number(number: u64, format: impl Fn(u64) -> Arc<str>) -> Arc<str> {
        let cache = NUMBER_DISPLAY_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &number) {
            return cached;
        }

        let formatted = format(number);
        Self::setter(cache, number, formatted.clone());
        formatted
    }

    pub(crate) fn permissions(mode: u32, format: impl Fn(u32) -> Arc<str>) -> Arc<str> {
        let cache = PERMISSIONS_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &mode) {
            return cached;
        }

        let formatted = format(mode);
        Self::setter(cache, mode, formatted.clone());
        formatted
    }

    pub(crate) fn size(bytes: u64, format: impl Fn(u64) -> Arc<str>) -> Arc<str> {
        let cache = SIZE_DISPLAY_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &bytes) {
            return cached;
        }

        let formatted = format(bytes);
        Self::setter(cache, bytes, formatted.clone());
        formatted
    }

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

    pub(crate) fn owner(uid: u32, lookup: impl Fn(u32) -> Arc<str>) -> Arc<str> {
        let cache = USER_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &uid) {
            return cached;
        }

        let formatted = lookup(uid);
        Self::setter(cache, uid, formatted.clone());
        formatted
    }

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

    pub(crate) fn group(gid: u32, lookup: impl Fn(u32) -> Arc<str>) -> Arc<str> {
        let cache = GROUP_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

        if let Some(cached) = Self::getter(cache, &gid) {
            return cached;
        }

        let formatted = lookup(gid);
        Self::setter(cache, gid, formatted.clone());
        formatted
    }

    /// Cache getter
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

    /// Cache setter
    fn setter<K: Eq + std::hash::Hash, V>(cache: &Mutex<HashMap<K, V>>, key: K, value: V) {
        if let Ok(mut map) = cache.lock() {
            map.insert(key, value);
        }
    }
}
