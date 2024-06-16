use rusql_core as rusql;

impl_database_ext! {
    rusql::postgres::Postgres {
        (),
        bool,
        String | &str,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        Vec<u8> | &[u8],

        rusql::postgres::types::Oid,

        rusql::postgres::types::PgInterval,

        rusql::postgres::types::PgMoney,

        rusql::postgres::types::PgLTree,

        rusql::postgres::types::PgLQuery,

        #[cfg(feature = "uuid")]
        rusql::types::Uuid,

        #[cfg(feature = "chrono")]
        rusql::types::chrono::NaiveTime,

        #[cfg(feature = "chrono")]
        rusql::types::chrono::NaiveDate,

        #[cfg(feature = "chrono")]
        rusql::types::chrono::NaiveDateTime,

        #[cfg(feature = "chrono")]
        rusql::types::chrono::DateTime<rusql::types::chrono::Utc> | rusql::types::chrono::DateTime<_>,

        #[cfg(feature = "chrono")]
        rusql::postgres::types::PgTimeTz<rusql::types::chrono::NaiveTime, rusql::types::chrono::FixedOffset>,

        #[cfg(feature = "time")]
        rusql::types::time::Time,

        #[cfg(feature = "time")]
        rusql::types::time::Date,

        #[cfg(feature = "time")]
        rusql::types::time::PrimitiveDateTime,

        #[cfg(feature = "time")]
        rusql::types::time::OffsetDateTime,

        #[cfg(feature = "time")]
        rusql::postgres::types::PgTimeTz<rusql::types::time::Time, rusql::types::time::UtcOffset>,

        #[cfg(feature = "bigdecimal")]
        rusql::types::BigDecimal,

        #[cfg(feature = "decimal")]
        rusql::types::Decimal,

        #[cfg(feature = "ipnetwork")]
        rusql::types::ipnetwork::IpNetwork,

        #[cfg(feature = "mac_address")]
        rusql::types::mac_address::MacAddress,

        #[cfg(feature = "json")]
        rusql::types::JsonValue,

        #[cfg(feature = "bit-vec")]
        rusql::types::BitVec,

        // Arrays

        Vec<bool> | &[bool],
        Vec<String> | &[String],
        Vec<Vec<u8>> | &[Vec<u8>],
        Vec<i8> | &[i8],
        Vec<i16> | &[i16],
        Vec<i32> | &[i32],
        Vec<i64> | &[i64],
        Vec<f32> | &[f32],
        Vec<f64> | &[f64],
        Vec<rusql::postgres::types::Oid> | &[rusql::postgres::types::Oid],
        Vec<rusql::postgres::types::PgMoney> | &[rusql::postgres::types::PgMoney],

        #[cfg(feature = "uuid")]
        Vec<rusql::types::Uuid> | &[rusql::types::Uuid],

        #[cfg(feature = "chrono")]
        Vec<rusql::types::chrono::NaiveTime> | &[rusql::types::chrono::NaiveTime],

        #[cfg(feature = "chrono")]
        Vec<rusql::types::chrono::NaiveDate> | &[rusql::types::chrono::NaiveDate],

        #[cfg(feature = "chrono")]
        Vec<rusql::types::chrono::NaiveDateTime> | &[rusql::types::chrono::NaiveDateTime],

        #[cfg(feature = "chrono")]
        Vec<rusql::types::chrono::DateTime<rusql::types::chrono::Utc>> | &[rusql::types::chrono::DateTime<_>],

        #[cfg(feature = "time")]
        Vec<rusql::types::time::Time> | &[rusql::types::time::Time],

        #[cfg(feature = "time")]
        Vec<rusql::types::time::Date> | &[rusql::types::time::Date],

        #[cfg(feature = "time")]
        Vec<rusql::types::time::PrimitiveDateTime> | &[rusql::types::time::PrimitiveDateTime],

        #[cfg(feature = "time")]
        Vec<rusql::types::time::OffsetDateTime> | &[rusql::types::time::OffsetDateTime],

        #[cfg(feature = "bigdecimal")]
        Vec<rusql::types::BigDecimal> | &[rusql::types::BigDecimal],

        #[cfg(feature = "decimal")]
        Vec<rusql::types::Decimal> | &[rusql::types::Decimal],

        #[cfg(feature = "ipnetwork")]
        Vec<rusql::types::ipnetwork::IpNetwork> | &[rusql::types::ipnetwork::IpNetwork],

        #[cfg(feature = "mac_address")]
        Vec<rusql::types::mac_address::MacAddress> | &[rusql::types::mac_address::MacAddress],

        #[cfg(feature = "json")]
        Vec<rusql::types::JsonValue> | &[rusql::types::JsonValue],

        // Ranges

        rusql::postgres::types::PgRange<i32>,
        rusql::postgres::types::PgRange<i64>,

        #[cfg(feature = "bigdecimal")]
        rusql::postgres::types::PgRange<rusql::types::BigDecimal>,

        #[cfg(feature = "decimal")]
        rusql::postgres::types::PgRange<rusql::types::Decimal>,

        #[cfg(feature = "chrono")]
        rusql::postgres::types::PgRange<rusql::types::chrono::NaiveDate>,

        #[cfg(feature = "chrono")]
        rusql::postgres::types::PgRange<rusql::types::chrono::NaiveDateTime>,

        #[cfg(feature = "chrono")]
        rusql::postgres::types::PgRange<rusql::types::chrono::DateTime<rusql::types::chrono::Utc>> |
            rusql::postgres::types::PgRange<rusql::types::chrono::DateTime<_>>,

        #[cfg(feature = "time")]
        rusql::postgres::types::PgRange<rusql::types::time::Date>,

        #[cfg(feature = "time")]
        rusql::postgres::types::PgRange<rusql::types::time::PrimitiveDateTime>,

        #[cfg(feature = "time")]
        rusql::postgres::types::PgRange<rusql::types::time::OffsetDateTime>,

        // Range arrays

        Vec<rusql::postgres::types::PgRange<i32>> | &[rusql::postgres::types::PgRange<i32>],
        Vec<rusql::postgres::types::PgRange<i64>> | &[rusql::postgres::types::PgRange<i64>],

        #[cfg(feature = "bigdecimal")]
        Vec<rusql::postgres::types::PgRange<rusql::types::BigDecimal>> |
            &[rusql::postgres::types::PgRange<rusql::types::BigDecimal>],

        #[cfg(feature = "decimal")]
        Vec<rusql::postgres::types::PgRange<rusql::types::Decimal>> |
            &[rusql::postgres::types::PgRange<rusql::types::Decimal>],

        #[cfg(feature = "chrono")]
        Vec<rusql::postgres::types::PgRange<rusql::types::chrono::NaiveDate>> |
            &[rusql::postgres::types::PgRange<rusql::types::chrono::NaiveDate>],

        #[cfg(feature = "chrono")]
        Vec<rusql::postgres::types::PgRange<rusql::types::chrono::NaiveDateTime>> |
            &[rusql::postgres::types::PgRange<rusql::types::chrono::NaiveDateTime>],

        #[cfg(feature = "chrono")]
        Vec<rusql::postgres::types::PgRange<rusql::types::chrono::DateTime<rusql::types::chrono::Utc>>> |
            Vec<rusql::postgres::types::PgRange<rusql::types::chrono::DateTime<_>>>,

        #[cfg(feature = "chrono")]
        &[rusql::postgres::types::PgRange<rusql::types::chrono::DateTime<rusql::types::chrono::Utc>>] |
            &[rusql::postgres::types::PgRange<rusql::types::chrono::DateTime<_>>],

        #[cfg(feature = "time")]
        Vec<rusql::postgres::types::PgRange<rusql::types::time::Date>> |
            &[rusql::postgres::types::PgRange<rusql::types::time::Date>],

        #[cfg(feature = "time")]
        Vec<rusql::postgres::types::PgRange<rusql::types::time::PrimitiveDateTime>> |
            &[rusql::postgres::types::PgRange<rusql::types::time::PrimitiveDateTime>],

        #[cfg(feature = "time")]
        Vec<rusql::postgres::types::PgRange<rusql::types::time::OffsetDateTime>> |
            &[rusql::postgres::types::PgRange<rusql::types::time::OffsetDateTime>],
    },
    ParamChecking::Strong,
    feature-types: info => info.__type_feature_gate(),
    row = rusql::postgres::PgRow,
    name = "PostgreSQL"
}
