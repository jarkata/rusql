use rusql_core as rusql;

// f32 is not included below as REAL represents a floating point value
// stored as an 8-byte IEEE floating point number
// For more info see: https://www.sqlite.org/datatype3.html#storage_classes_and_datatypes
impl_database_ext! {
    rusql::sqlite::Sqlite {
        bool,
        i32,
        i64,
        f64,
        String,
        Vec<u8>,

        #[cfg(feature = "chrono")]
        rusql::types::chrono::NaiveDateTime,

        #[cfg(feature = "chrono")]
        rusql::types::chrono::DateTime<rusql::types::chrono::Utc> | rusql::types::chrono::DateTime<_>,
    },
    ParamChecking::Weak,
    feature-types: _info => None,
    row = rusql::sqlite::SqliteRow,
    name = "SQLite"
}
