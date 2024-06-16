use crate::any::{Any, AnyTypeInfo};
use crate::column::{Column, ColumnIndex};

#[cfg(feature = "postgres")]
use crate::postgres::{PgColumn, PgRow, PgStatement};

#[cfg(feature = "mysql")]
use crate::mysql::{MySqlColumn, MySqlRow, MySqlStatement};

#[cfg(feature = "sqlite")]
use crate::sqlite::{SqliteColumn, SqliteRow, SqliteStatement};

#[derive(Debug, Clone)]
pub struct AnyColumn {
    pub(crate) kind: AnyColumnKind,
    pub(crate) type_info: AnyTypeInfo,
}

impl crate::column::private_column::Sealed for AnyColumn {}

#[derive(Debug, Clone)]
pub(crate) enum AnyColumnKind {
    #[cfg(feature = "postgres")]
    Postgres(PgColumn),

    #[cfg(feature = "mysql")]
    MySql(MySqlColumn),

    #[cfg(feature = "sqlite")]
    Sqlite(SqliteColumn),
}

impl Column for AnyColumn {
    type Database = Any;

    fn ordinal(&self) -> usize {
        match &self.kind {
            #[cfg(feature = "postgres")]
            AnyColumnKind::Postgres(row) => row.ordinal(),

            #[cfg(feature = "mysql")]
            AnyColumnKind::MySql(row) => row.ordinal(),

            #[cfg(feature = "sqlite")]
            AnyColumnKind::Sqlite(row) => row.ordinal(),
        }
    }

    fn name(&self) -> &str {
        match &self.kind {
            #[cfg(feature = "postgres")]
            AnyColumnKind::Postgres(row) => row.name(),

            #[cfg(feature = "mysql")]
            AnyColumnKind::MySql(row) => row.name(),

            #[cfg(feature = "sqlite")]
            AnyColumnKind::Sqlite(row) => row.name(),
        }
    }

    fn type_info(&self) -> &AnyTypeInfo {
        &self.type_info
    }
}

// FIXME: Find a nice way to auto-generate the below or petition Rust to add support for #[cfg]
//        to trait bounds

// all 4

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "sqlite"
))]
pub trait AnyColumnIndex:
ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>> + ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>> + ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>>
{}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "sqlite"
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>> + ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>> + ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>>
{}

#[cfg(all(
    not(feature = "mysql"),
    all(feature = "postgres", feature = "sqlite")
))]
pub trait AnyColumnIndex:
ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>> + ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>>
{}

#[cfg(all(
    not(feature = "mysql"),
    all(feature = "postgres", feature = "sqlite")
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>> + ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>>
{}

#[cfg(all(
    not(feature = "sqlite"),
    all(feature = "postgres", feature = "mysql")
))]
pub trait AnyColumnIndex:
ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>> + ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>>
{}

#[cfg(all(
    not(feature = "sqlite"),
    all(feature = "postgres", feature = "mysql")
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>> + ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>>
{}

#[cfg(all(
    not(feature = "postgres"),
    all(feature = "sqlite", feature = "mysql")
))]
pub trait AnyColumnIndex:
ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>> + ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>>
{}

#[cfg(all(
    not(feature = "postgres"),
    all(feature = "sqlite", feature = "mysql")
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>> + ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    all(feature = "postgres")
))]
pub trait AnyColumnIndex:
ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    all(feature = "postgres")
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<PgRow> + for<'q> ColumnIndex<PgStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    all(feature = "mysql")
))]
pub trait AnyColumnIndex:
ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    all(feature = "mysql")
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<MySqlRow> + for<'q> ColumnIndex<MySqlStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "mysql")),
    all(feature = "sqlite")
))]
pub trait AnyColumnIndex:
ColumnIndex<SqliteRow>
+ for<'q> ColumnIndex<SqliteStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "mysql")),
    all(feature = "sqlite")
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<SqliteRow>
    + for<'q> ColumnIndex<SqliteStatement<'q>>
{}


#[cfg(all(
    not(any(feature = "mysql", feature = "postgres")),
    feature = "sqlite"
))]
pub trait AnyColumnIndex:
ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "postgres")),
    feature = "sqlite"
))]
impl<I: ?Sized> AnyColumnIndex for I where
    I: ColumnIndex<SqliteRow> + for<'q> ColumnIndex<SqliteStatement<'q>>
{}
