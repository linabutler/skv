/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use rusqlite::{types::FromSql, OptionalExtension, ToSql};

pub struct Dao<'a> {
    conn: &'a rusqlite::Connection,
}

impl<'a> Dao<'a> {
    pub fn new(conn: &'a rusqlite::Connection) -> Self {
        Self { conn }
    }

    /// Returns the value associated with a metadata key.
    pub fn get_meta<T: FromSql>(&self, key: &str) -> Result<Option<T>, rusqlite::Error> {
        self.conn
            .query_row(
                "SELECT value FROM meta WHERE key = :key",
                rusqlite::named_params! { ":key": key },
                |row| row.get::<_, T>(0),
            )
            .optional()
    }

    /// Sets the value for a metadata key.
    pub fn put_meta(&self, key: &str, value: impl ToSql) -> Result<(), rusqlite::Error> {
        self.conn.execute(
            "INSERT INTO meta(key, value) VALUES(:key, :value)
             ON CONFLICT DO UPDATE SET
               value = excluded.value",
            rusqlite::named_params! { ":key": key, ":value": value },
        )?;
        Ok(())
    }
}
