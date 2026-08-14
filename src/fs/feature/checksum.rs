// SPDX-License-Identifier: MIT

#[cfg(feature = "checksum")]
use crate::cli::flags::HashAlgorithm;

#[cfg(feature = "checksum")]
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

#[cfg(feature = "checksum")]
use std::fs;

#[cfg(feature = "checksum")]
use std::io::{self, Read};

#[cfg(feature = "checksum")]
use std::path::Path;

#[cfg(feature = "checksum")]
use crc32fast::Hasher;

#[cfg(feature = "checksum")]
use std::sync::Arc;

#[cfg(feature = "checksum")]
/// Hashes a file and hex-encodes the digest.
///
/// # Parameters
/// - `path`: The file to hash.
/// - `algorithm`: The hash algorithm to use.
///
/// # Returns
/// The hex-encoded digest, or `"-"` for directories and unreadable files.
pub(crate) fn compute(path: &Path, algorithm: HashAlgorithm) -> Arc<str> {
    if path.is_dir() {
        return "-".into();
    }

    match digest(path, algorithm) {
        Ok(hash) => hash.into(),
        Err(_) => "-".into(),
    }
}

#[cfg(feature = "checksum")]
/// Runs the selected algorithm over the file.
///
/// # Parameters
/// - `path`: The file to hash.
/// - `algorithm`: The hash algorithm to use.
///
/// # Returns
/// The hex-encoded digest, or an I/O error if the file cannot be read.
fn digest(path: &Path, algorithm: HashAlgorithm) -> io::Result<String> {
    match algorithm {
        HashAlgorithm::Md5 => {
            let mut context = md5::Context::new();
            stream(path, |chunk| context.consume(chunk))?;
            Ok(format!("{:x}", context.finalize()))
        }
        HashAlgorithm::Crc32 => {
            let mut hasher = Hasher::new();
            stream(path, |chunk| hasher.update(chunk))?;
            Ok(format!("{:08x}", hasher.finalize()))
        }
        HashAlgorithm::Sha224 => sha::<Sha224>(path),
        HashAlgorithm::Sha256 => sha::<Sha256>(path),
        HashAlgorithm::Sha384 => sha::<Sha384>(path),
        HashAlgorithm::Sha512 => sha::<Sha512>(path),
    }
}

#[cfg(feature = "checksum")]
/// Hex-encodes the SHA-2 digest of the file for the given variant.
///
/// # Parameters
/// - `path`: The file to hash.
fn sha<D: Digest>(path: &Path) -> io::Result<String> {
    let mut hasher = D::new();
    stream(path, |chunk| hasher.update(chunk))?;
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

#[cfg(feature = "checksum")]
/// Reads the file in chunks, handing each to `consume`.
///
/// # Parameters
/// - `path`: The file to read.
/// - `consume`: Called with every chunk read, in order.
fn stream(path: &Path, mut consume: impl FnMut(&[u8])) -> io::Result<()> {
    let mut file = fs::File::open(path)?;
    let mut buffer = [0u8; 8192];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            return Ok(());
        }
        consume(&buffer[..read]);
    }
}
