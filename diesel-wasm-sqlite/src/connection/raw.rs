use crate::{
    sqlite_types::{SqliteFlags, SqliteOpenFlags},
    SqliteType, WasmSqlite, WasmSqliteError,
};
use diesel::{
    connection::statement_cache::PrepareForCache, result::*, serialize::ToSql,
    sql_types::HasSqlType,
};
use tokio::sync::oneshot;
use wasm_bindgen::{closure::Closure, JsValue};

use super::stmt::Statement;

#[allow(missing_copy_implementations)]
#[derive(Debug)]
pub(super) struct RawConnection {
    pub(super) internal_connection: JsValue,
    drop_signal: Option<oneshot::Sender<JsValue>>,
}

impl Clone for RawConnection {
    fn clone(&self) -> Self {
        Self {
            internal_connection: self.internal_connection.clone(),
            drop_signal: None,
        }
    }
}

impl RawConnection {
    pub(super) async fn establish(database_url: &str) -> ConnectionResult<Self> {
        let sqlite3 = crate::get_sqlite().await;
        let database_url = if database_url.starts_with("sqlite://") {
            database_url.replacen("sqlite://", "file:", 1)
        } else {
            database_url.to_string()
        };
        let flags = SqliteOpenFlags::SQLITE_OPEN_READWRITE
            | SqliteOpenFlags::SQLITE_OPEN_CREATE
            | SqliteOpenFlags::SQLITE_OPEN_URI;

        let (tx, rx) = oneshot::channel::<JsValue>();
        // TODO: can make this into function/macro
        wasm_bindgen_futures::spawn_local(async move {
            match rx.await {
                Ok(conn) => {
                    let sqlite3 = crate::get_sqlite_unchecked();
                    match sqlite3.close(&conn).await {
                        Ok(_) => log::debug!("db closed"),
                        Err(e) => {
                            log::error!("error during db close");
                            web_sys::console::log_1(&e);
                        }
                    }
                }
                Err(_) => {
                    log::error!("RawConnection never dropped.");
                }
            }
        });

        Ok(RawConnection {
            internal_connection: sqlite3
                .open_v2(&database_url, Some(flags.bits() as i32))
                .await
                .map_err(WasmSqliteError::from)
                .map_err(ConnectionError::from)?,
            drop_signal: Some(tx),
        })
    }

    pub(super) async fn exec(&self, query: &str) -> QueryResult<()> {
        let sqlite3 = crate::get_sqlite().await;
        let result = sqlite3
            .exec(&self.internal_connection, query)
            .await
            .unwrap();

        Ok(result)
    }

    pub(super) fn rows_affected_by_last_query(&self) -> usize {
        let sqlite3 = crate::get_sqlite_unchecked();
        sqlite3.changes(&self.internal_connection)
    }

    pub(super) fn register_sql_function<F, Ret, RetSqlType>(
        &self,
        fn_name: &str,
        num_args: usize,
        deterministic: bool,
        f: F,
    ) -> QueryResult<()>
    where
        F: FnMut(JsValue, Vec<JsValue>) -> JsValue + 'static,
        Ret: ToSql<RetSqlType, WasmSqlite>,
        WasmSqlite: HasSqlType<RetSqlType>,
    {
        let sqlite3 = crate::get_sqlite_unchecked();
        let flags = Self::get_flags(deterministic);

        let cb = Closure::new(f);
        sqlite3
            .create_function(
                &self.internal_connection,
                fn_name,
                num_args
                    .try_into()
                    .expect("usize to i32 panicked in register_sql_function"),
                flags,
                0,
                Some(&cb),
                None,
                None,
            )
            .unwrap();
        Ok(())
    }

    fn get_flags(deterministic: bool) -> i32 {
        let mut flags = SqliteFlags::SQLITE_UTF8;
        if deterministic {
            flags |= SqliteFlags::SQLITE_DETERMINISTIC;
        }
        flags.bits() as i32
    }

    /* possible to implement this, but would need to fill in the missing wa-sqlite functions
    pub(super) fn serialize(&mut self) -> SerializedDatabase {
        unsafe {
            let mut size: ffi::sqlite3_int64 = 0;
            let data_ptr = ffi::sqlite3_serialize(
                self.internal_connection.as_ptr(),
                std::ptr::null(),
                &mut size as *mut _,
                0,
            );
            SerializedDatabase::new(data_ptr, size as usize)
        }
    }

    pub(super) fn deserialize(&mut self, data: &[u8]) -> QueryResult<()> {
        // the cast for `ffi::SQLITE_DESERIALIZE_READONLY` is required for old libsqlite3-sys versions
        #[allow(clippy::unnecessary_cast)]
        unsafe {
            let result = ffi::sqlite3_deserialize(
                self.internal_connection.as_ptr(),
                std::ptr::null(),
                data.as_ptr() as *mut u8,
                data.len() as i64,
                data.len() as i64,
                ffi::SQLITE_DESERIALIZE_READONLY as u32,
            );

            ensure_sqlite_ok(result, self.internal_connection.as_ptr())
        }
    }
    */
}

#[async_trait::async_trait(?Send)]
impl diesel_async::stmt_cache::PrepareCallback<Statement, SqliteType> for RawConnection {
    async fn prepare(
        self,
        sql: &str,
        _metadata: &[SqliteType],
        is_for_cache: PrepareForCache,
    ) -> QueryResult<(Statement, Self)> {
        let stmt = Statement::prepare(&self, sql, is_for_cache).await;
        Ok((stmt?, self))
    }
}

impl Drop for RawConnection {
    fn drop(&mut self) {
        if let Some(s) = self.drop_signal.take() {
            let _ = s.send(self.internal_connection.clone());
        } else {
            log::warn!("RawConnection not dropped because drop_signal is empty");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection::{AsyncConnection, WasmSqliteConnection};
    use diesel::connection::Connection;
    use wasm_bindgen_test::*;
    use web_sys::console;
    wasm_bindgen_test_configure!(run_in_dedicated_worker);

    #[wasm_bindgen_test]
    async fn test_fn_registration() {
        let mut result = WasmSqliteConnection::establish("test").await;
        let mut conn = result.unwrap();
        console::log_1(&"CONNECTED".into());
        conn.raw
            .register_sql_function("test", 0, true, |ctx, values| {
                console::log_1(&"Inside Fn".into());
            });
    }
}
