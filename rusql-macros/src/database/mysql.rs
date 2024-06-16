use rusql_core as rusql;

impl_database_ext! {
    rusql::mysql::MySql {
        u8,
        u16,
        u32,
        u64,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,

        // ordering is important here as otherwise we might infer strings to be binary
        // CHAR, VAR_CHAR, TEXT
        String,

        // BINARY, VAR_BINARY, BLOB
        Vec<u8>,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        rusql::types::chrono::NaiveTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        rusql::types::chrono::NaiveDate,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        rusql::types::chrono::NaiveDateTime,

        #[cfg(all(feature = "chrono", not(feature = "time")))]
        rusql::types::chrono::DateTime<rusql::types::chrono::Utc>,

        #[cfg(feature = "time")]
        rusql::types::time::Time,

        #[cfg(feature = "time")]
        rusql::types::time::Date,

        #[cfg(feature = "time")]
        rusql::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        rusql::types::time::OffsetDateTime,

        #[cfg(feature = "bigdecimal")]
        rusql::types::BigDecimal,

        #[cfg(feature = "decimal")]
        rusql::types::Decimal,

        #[cfg(feature = "json")]
        rusql::types::JsonValue,
    },
    ParamChecking::Weak,
    feature-types: info => info.__type_feature_gate(),
    row = rusql::mysql::MySqlRow,
    name = "MySQL"
}
