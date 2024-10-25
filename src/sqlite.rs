/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::{
    os::raw::c_void,
    ptr::{self, NonNull},
};

/// Wraps a [`rusqlite::Connection`] with operations that
/// access the underlying database file.
///
/// Rusqlite doesn't expose higher-level interfaces for these operations
/// (yet?), so we muck about with the raw, unsafe connection handle.
pub struct SqliteDatabaseFile<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> SqliteDatabaseFile<'a> {
    pub fn for_connection(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn size(&self) -> Option<u64> {
        let file = unsafe { get_sqlite3_file(self.conn.handle()) }?;
        let methods = NonNull::<rusqlite::ffi::sqlite3_io_methods>::new(
            unsafe { *file.as_ptr() }.pMethods as *mut _,
        )?;
        let f = unsafe { *methods.as_ptr() }.xFileSize?;
        let mut size = 0i64;
        let rc = unsafe { f(file.as_ptr(), &mut size) };
        match rc {
            rusqlite::ffi::SQLITE_OK => u64::try_from(size).ok(),
            _ => None,
        }
    }

    pub fn read_exact_at(&self, buf: &mut [u8], offset: u64) -> Result<(), rusqlite::Error> {
        let Some(file) = (unsafe { get_sqlite3_file(self.conn.handle()) }) else {
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                None,
            ));
        };
        let Some(methods) = NonNull::<rusqlite::ffi::sqlite3_io_methods>::new(
            unsafe { *file.as_ptr() }.pMethods as *mut _,
        ) else {
            // In-memory databases don't have any `sqlite3_io_methods`.
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                None,
            ));
        };
        let Some(f) = unsafe { *methods.as_ptr() }.xRead else {
            // `xRead` should always be implemented, but it's nullable in
            // the generated bindings.
            return Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_MISUSE),
                None,
            ));
        };
        let rc = unsafe {
            f(
                file.as_ptr(),
                buf.as_mut_ptr() as *mut c_void,
                buf.len() as i32,
                offset as i64,
            )
        };
        match rc {
            rusqlite::ffi::SQLITE_OK => Ok(()),
            code => Err(rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(code),
                None,
            )),
        }
    }
}

unsafe fn get_sqlite3_file(
    handle: *mut rusqlite::ffi::sqlite3,
) -> Option<NonNull<rusqlite::ffi::sqlite3_file>> {
    let mut file = std::ptr::null_mut::<rusqlite::ffi::sqlite3_file>();
    let rc = rusqlite::ffi::sqlite3_file_control(
        handle,
        ptr::null(), // Defaults to the `main` database.
        rusqlite::ffi::SQLITE_FCNTL_FILE_POINTER,
        &mut file as *mut *mut rusqlite::ffi::sqlite3_file as *mut c_void,
    );
    if rc != rusqlite::ffi::SQLITE_OK {
        // Should never fail.
        return None;
    }
    NonNull::new(file)
}
