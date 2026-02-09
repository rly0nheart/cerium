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

use crate::cli::flags::OwnershipFormat;
use libc::{c_char, getgrgid_r, getpwuid_r, gid_t, group, passwd, uid_t};
use std::ffi::CStr;
use std::mem;
use std::ptr;
use std::sync::Arc;

/// Resolved user identity with name and UID.
#[derive(Debug)]
struct User {
    pub name: String,
    pub uid: uid_t,
}

/// Resolved group identity with name and GID.
#[derive(Debug)]
struct Group {
    pub name: String,
    pub gid: gid_t,
}

/// Formats user and group ownership according to the selected [`OwnershipFormat`].
pub(crate) struct Ownership {
    ownership_format: OwnershipFormat,
}

impl Ownership {
    /// Creates a new [`Ownership`] formatter.
    ///
    /// # Parameters
    /// - `ownership_format`: Whether to display name or numeric ID.
    pub(crate) fn new(ownership_format: OwnershipFormat) -> Self {
        Self { ownership_format }
    }

    /// Looks up a user by UID via `getpwuid_r`.
    ///
    /// # Parameters
    /// - `user_id`: The UID to resolve.
    ///
    /// # Returns
    /// A [`User`] with the resolved name, or the numeric UID as a fallback.
    fn user_by_uid(user_id: uid_t) -> User {
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
                return User {
                    name: CStr::from_ptr(passwd_entry.pw_name)
                        .to_string_lossy()
                        .into_owned(),
                    uid: passwd_entry.pw_uid,
                };
            }
        }

        User {
            name: user_id.to_string(),
            uid: user_id,
        }
    }

    /// Looks up a group by GID via `getgrgid_r`.
    ///
    /// # Parameters
    /// - `group_id`: The GID to resolve.
    ///
    /// # Returns
    /// A [`Group`] with the resolved name, or the numeric GID as a fallback.
    fn group_by_gid(group_id: gid_t) -> Group {
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
                return Group {
                    name: CStr::from_ptr(group_entry.gr_name)
                        .to_string_lossy()
                        .into_owned(),
                    gid: group_entry.gr_gid,
                };
            }
        }

        Group {
            name: group_id.to_string(),
            gid: group_id,
        }
    }
    /// Formats a UID as either a username or numeric ID.
    ///
    /// # Parameters
    /// - `uid`: The user ID to format.
    pub(crate) fn format_user(&self, uid: u32) -> Arc<str> {
        let user = Self::user_by_uid(uid);
        match self.ownership_format {
            OwnershipFormat::Name => user.name.into(),
            OwnershipFormat::Id => user.uid.to_string().into(),
        }
    }

    /// Formats a GID as either a group name or numeric ID.
    ///
    /// # Parameters
    /// - `gid`: The group ID to format.
    pub(crate) fn format_group(&self, gid: u32) -> Arc<str> {
        let group = Self::group_by_gid(gid);
        match self.ownership_format {
            OwnershipFormat::Name => group.name.into(),
            OwnershipFormat::Id => group.gid.to_string().into(),
        }
    }
}
