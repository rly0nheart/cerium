// SPDX-License-Identifier: MIT

use crate::fs::xattr;
use std::path::Path;
use std::sync::Arc;

/// The extended attribute a POSIX ACL is stored under.
const ACL_XATTR: &[u8] = b"system.posix_acl_access";

/// Checks if a file has ACLs beyond standard Unix permissions.
///
/// # Parameters
/// - `path`: Path to the file to inspect.
///
/// # Returns
/// `"+"` if ACLs are present, `"-"` if none or on error.
pub(crate) fn indicator(path: &Path) -> Arc<str> {
    let has_acl = xattr::list_names(path)
        .split(|&byte| byte == 0)
        .any(|name| name == ACL_XATTR);

    if has_acl { "+".into() } else { "-".into() }
}
