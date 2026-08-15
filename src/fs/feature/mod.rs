#[cfg(all(feature = "magic", not(target_os = "android")))]
pub(crate) mod magic;
