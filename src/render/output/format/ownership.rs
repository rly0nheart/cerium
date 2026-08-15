// SPDX-License-Identifier: MIT

use crate::cli::flags::OwnershipFormat;
use libc::{c_char, getgrgid_r, getpwuid_r, gid_t, group, passwd, uid_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::sync::Arc;

/// Formats a UID as either a username or numeric ID.
///
/// # Parameters
/// - `mode`: Whether to display the name or the numeric ID.
/// - `uid`: The user ID to format.
pub(crate) fn user(mode: OwnershipFormat, uid: u32) -> Arc<str> {
    match mode {
        OwnershipFormat::Name => user_by_uid(uid).into(),
        OwnershipFormat::Id => uid.to_string().into(),
    }
}

/// Formats a GID as either a group name or numeric ID.
///
/// # Parameters
/// - `mode`: Whether to display the name or the numeric ID.
/// - `gid`: The group ID to format.
pub(crate) fn group(mode: OwnershipFormat, gid: u32) -> Arc<str> {
    match mode {
        OwnershipFormat::Name => group_by_gid(gid).into(),
        OwnershipFormat::Id => gid.to_string().into(),
    }
}

/// Looks up a user by UID via `getpwuid_r`.
///
/// # Parameters
/// - `user_id`: The UID to resolve.
///
/// # Returns
/// The resolved username, or the numeric UID as a fallback.
fn user_by_uid(user_id: uid_t) -> String {
    unsafe {
        let mut passwd_entry: passwd = mem::zeroed();
        let mut passwd_result: *mut passwd = ptr::null_mut();
        let mut buffer = vec![0u8; 16 * 1024];

        // c_char is i8 on most platforms but u8 on Android
        let status = getpwuid_r(
            user_id,
            &mut passwd_entry,
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
            &mut passwd_result,
        );

        if status == 0 && !passwd_result.is_null() && !passwd_entry.pw_name.is_null() {
            return CStr::from_ptr(passwd_entry.pw_name)
                .to_string_lossy()
                .into_owned();
        }
    }

    user_id.to_string()
}

/// Looks up a group by GID via `getgrgid_r`.
///
/// # Parameters
/// - `group_id`: The GID to resolve.
///
/// # Returns
/// The resolved group name, or the numeric GID as a fallback.
fn group_by_gid(group_id: gid_t) -> String {
    unsafe {
        let mut group_entry: group = mem::zeroed();
        let mut group_result: *mut group = ptr::null_mut();
        let mut buffer = vec![0u8; 16 * 1024];

        // c_char is i8 on most platforms but u8 on Android
        let status = getgrgid_r(
            group_id,
            &mut group_entry,
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
            &mut group_result,
        );

        if status == 0 && !group_result.is_null() && !group_entry.gr_name.is_null() {
            return CStr::from_ptr(group_entry.gr_name)
                .to_string_lossy()
                .into_owned();
        }
    }

    group_id.to_string()
}
