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
pub struct Checksum<'a> {
    path: &'a Path,
    algorithm: HashAlgorithm,
}

#[cfg(feature = "checksum")]
impl<'a> Checksum<'a> {
    pub(crate) fn new(path: &'a Path, algorithm: HashAlgorithm) -> Self {
        Self { path, algorithm }
    }

    /// Compute checksum for the file
    pub(crate) fn compute(&self) -> Arc<str> {
        // Skip directories
        if self.path.is_dir() {
            return "-".into();
        }

        match self.compute_hash() {
            Ok(hash) => hash.into(),
            Err(_) => "-".into(),
        }
    }

    /// Generic hash computation
    fn compute_hash(&self) -> io::Result<String> {
        match self.algorithm {
            HashAlgorithm::Md5 => {
                let data = fs::read(self.path)?;
                let digest = md5::compute(&data);
                Ok(format!("{:x}", digest))
            }
            HashAlgorithm::Crc32 => {
                let mut hasher = Hasher::new();
                let mut file = fs::File::open(self.path)?;
                let mut buffer = [0u8; 8192];
                loop {
                    let n = file.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                Ok(format!("{:08x}", hasher.finalize()))
            }
            HashAlgorithm::Sha224 => {
                let mut hasher = Sha224::new();
                let mut file = fs::File::open(self.path)?;
                let mut buffer = [0u8; 8192];
                loop {
                    let n = file.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                Ok(format!("{:x}", hasher.finalize()))
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                let mut file = fs::File::open(self.path)?;
                let mut buffer = [0u8; 8192];
                loop {
                    let n = file.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                Ok(format!("{:x}", hasher.finalize()))
            }
            HashAlgorithm::Sha384 => {
                let mut hasher = Sha384::new();
                let mut file = fs::File::open(self.path)?;
                let mut buffer = [0u8; 8192];
                loop {
                    let n = file.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                Ok(format!("{:x}", hasher.finalize()))
            }
            HashAlgorithm::Sha512 => {
                let mut hasher = Sha512::new();
                let mut file = fs::File::open(self.path)?;
                let mut buffer = [0u8; 8192];
                loop {
                    let n = file.read(&mut buffer)?;
                    if n == 0 {
                        break;
                    }
                    hasher.update(&buffer[..n]);
                }
                Ok(format!("{:x}", hasher.finalize()))
            }
        }
    }
}
