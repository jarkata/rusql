#![cfg_attr(docsrs, feature(doc_cfg))]

// #[cfg(any(feature = "runtime-async-std", feature = "runtime-tokio"))]
// compile_error!(
//     "the features 'runtime-actix', 'runtime-async-std' and 'runtime-tokio' have been removed in
//      favor of new features 'runtime-{rt}-{tls}' where rt is one of 'async-std' and 'tokio'
//      and 'tls' is one of 'native-tls' and 'rustls'."
// );

pub use rusql_core::acquire::Acquire;
pub use rusql_core::arguments::{Arguments, IntoArguments};
pub use rusql_core::column::Column;
pub use rusql_core::column::ColumnIndex;
pub use rusql_core::connection::{ConnectOptions, Connection};
pub use rusql_core::database::{self, Database};
pub use rusql_core::describe::Describe;
pub use rusql_core::executor::{Execute, Executor};
pub use rusql_core::from_row::FromRow;
pub use rusql_core::pool::{self, Pool};
pub use rusql_core::query::{query, query_with, execute_update};
pub use rusql_core::query_as::{query_as, query_as_with};
pub use rusql_core::query_builder::{self, QueryBuilder};
pub use rusql_core::query_scalar::{query_scalar, query_scalar_with};
pub use rusql_core::row::Row;
pub use rusql_core::statement::Statement;
pub use rusql_core::transaction::{Transaction, TransactionManager};
pub use rusql_core::type_info::TypeInfo;
pub use rusql_core::types::Type;
pub use rusql_core::value::{Value, ValueRef};
pub use rusql_core::Either;

#[doc(inline)]
pub use rusql_core::error::{self, Error, Result};

#[cfg(feature = "migrate")]
pub use rusql_core::migrate;

#[cfg(all(
    any(
        feature = "mysql",
        feature = "sqlite",
        feature = "postgres",
    ),
    feature = "any"
))]
pub use rusql_core::any::{self, Any, AnyConnection, AnyExecutor, AnyPool};

#[cfg(feature = "mysql")]
#[cfg_attr(docsrs, doc(cfg(feature = "mysql")))]
pub use rusql_core::mysql::{self, MySql, MySqlConnection, MySqlExecutor, MySqlPool};

#[cfg(feature = "postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "postgres")))]
pub use rusql_core::postgres::{self, PgConnection, PgExecutor, PgPool, Postgres};

#[cfg(feature = "sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlite")))]
pub use rusql_core::sqlite::{self, Sqlite, SqliteConnection, SqliteExecutor, SqlitePool};

#[cfg(feature = "macros")]
#[doc(hidden)]
pub extern crate rusql_macros;

// derives
#[cfg(feature = "macros")]
#[doc(hidden)]
pub use rusql_macros::{FromRow, Type};

// We can't do our normal facade approach with an attribute, but thankfully we can now
// have docs out-of-line quite easily.
#[doc = include_str!("macros/test.md")]
pub use rusql_macros::test;


#[cfg(feature = "macros")]
mod macros;

// macro support
#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod ty_match;

/// Conversions between Rust and SQL types.
///
/// To see how each SQL type maps to a Rust type, see the corresponding `types` module for each
/// database:
///
///  * Postgres: [postgres::types]
///  * MySQL: [mysql::types]
///  * SQLite: [sqlite::types]
///
/// Any external types that have had [`Type`] implemented for, are re-exported in this module
/// for convenience as downstream users need to use a compatible version of the external crate
/// to take advantage of the implementation.
///
/// [`Type`]: types::Type
pub mod types {
    pub use rusql_core::types::*;

    #[cfg(feature = "macros")]
    #[doc(hidden)]
    pub use rusql_macros::Type;
}

/// Provides [`Encode`](encode::Encode) for encoding values for the database.
pub mod encode {
    pub use rusql_core::encode::{Encode, IsNull};

    #[cfg(feature = "macros")]
    #[doc(hidden)]
    pub use rusql_macros::Encode;
}

pub use self::encode::Encode;

/// Provides [`Decode`](decode::Decode) for decoding values from the database.
pub mod decode {
    pub use rusql_core::decode::Decode;

    #[cfg(feature = "macros")]
    #[doc(hidden)]
    pub use rusql_macros::Decode;
}

pub use self::decode::Decode;

/// Types and traits for the `query` family of functions and macros.
pub mod query {
    pub use rusql_core::query::{Map, Query};
    pub use rusql_core::query_as::QueryAs;
    pub use rusql_core::query_scalar::QueryScalar;
}

/// Convenience re-export of common traits.
pub mod prelude {
    pub use super::Acquire;
    pub use super::ConnectOptions;
    pub use super::Connection;
    pub use super::Decode;
    pub use super::Encode;
    pub use super::Executor;
    pub use super::FromRow;
    pub use super::IntoArguments;
    pub use super::Row;
    pub use super::Statement;
    pub use super::Type;
}

#[doc(hidden)]
#[inline(always)]
#[deprecated = "`#[rusql(rename = \"...\")]` is now `#[rusql(type_name = \"...\")`"]
pub fn _rename() {}
