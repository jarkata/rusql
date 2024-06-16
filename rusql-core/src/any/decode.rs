use crate::decode::Decode;
use crate::types::Type;

#[cfg(feature = "postgres")]
use crate::postgres::Postgres;

#[cfg(feature = "mysql")]
use crate::mysql::MySql;

#[cfg(feature = "sqlite")]
use crate::sqlite::Sqlite;

// Implements Decode for any T where T supports Decode for any database that has support currently
// compiled into RuSQL
macro_rules! impl_any_decode {
    ($ty:ty) => {
        impl<'r> crate::decode::Decode<'r, crate::any::Any> for $ty
        where
            $ty: crate::any::AnyDecode<'r>,
        {
            fn decode(
                value: crate::any::AnyValueRef<'r>,
            ) -> Result<Self, crate::error::BoxDynError> {
                match value.kind {
                    #[cfg(feature = "mysql")]
                    crate::any::value::AnyValueRefKind::MySql(value) => {
                        <$ty as crate::decode::Decode<'r, crate::mysql::MySql>>::decode(value)
                    }

                    #[cfg(feature = "sqlite")]
                    crate::any::value::AnyValueRefKind::Sqlite(value) => {
                        <$ty as crate::decode::Decode<'r, crate::sqlite::Sqlite>>::decode(value)
                    }

                    #[cfg(feature = "postgres")]
                    crate::any::value::AnyValueRefKind::Postgres(value) => {
                        <$ty as crate::decode::Decode<'r, crate::postgres::Postgres>>::decode(value)
                    }
                }
            }
        }
    };
}

// FIXME: Find a nice way to auto-generate the below or petition Rust to add support for #[cfg]
//        to trait bounds

// all 4

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "sqlite"
))]
pub trait AnyDecode<'r>:
Decode<'r, Postgres>
+ Type<Postgres>
+ Decode<'r, MySql>
+ Type<MySql>
+ Decode<'r, Sqlite>
+ Type<Sqlite>
{}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "sqlite"
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Postgres>
    + Type<Postgres>
    + Decode<'r, MySql>
    + Type<MySql>
    + Decode<'r, Sqlite>
    + Type<Sqlite>
{}

#[cfg(all(
    not(feature = "mysql"),
    all(feature = "postgres", feature = "sqlite")
))]
pub trait AnyDecode<'r>:
Decode<'r, Postgres>
+ Type<Postgres>
+ Decode<'r, Sqlite>
+ Type<Sqlite>
{}

#[cfg(all(
    not(feature = "mysql"),
    all(feature = "postgres", feature = "sqlite")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Postgres>
    + Type<Postgres>
    + Decode<'r, Sqlite>
    + Type<Sqlite>
{}

#[cfg(all(
    not(feature = "sqlite"),
    all(feature = "postgres", feature = "mysql")
))]
pub trait AnyDecode<'r>:
Decode<'r, Postgres>
+ Type<Postgres>
+ Decode<'r, MySql>
+ Type<MySql>
{}

#[cfg(all(
    not(feature = "sqlite"),
    all(feature = "postgres", feature = "mysql")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Postgres>
    + Type<Postgres>
    + Decode<'r, MySql>
    + Type<MySql>
{}

#[cfg(all(
    not(feature = "postgres"),
    all(feature = "sqlite", feature = "mysql")
))]
pub trait AnyDecode<'r>:
Decode<'r, Sqlite>
+ Type<Sqlite>
+ Decode<'r, MySql>
+ Type<MySql>
{}

#[cfg(all(
    not(feature = "postgres"),
    all(feature = "sqlite", feature = "mysql")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Sqlite>
    + Type<Sqlite>
    + Decode<'r, MySql>
    + Type<MySql>
{}

// only 2 (6)
#[cfg(all(
    not(any(feature = "sqlite")),
    all(feature = "postgres", feature = "mysql")
))]
pub trait AnyDecode<'r>:
Decode<'r, Postgres> + Type<Postgres> + Decode<'r, MySql> + Type<MySql>
{}

#[cfg(all(
    not(any(feature = "sqlite")),
    all(feature = "postgres", feature = "mysql")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Postgres> + Type<Postgres> + Decode<'r, MySql> + Type<MySql>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    all(feature = "postgres")
))]
pub trait AnyDecode<'r>:
Decode<'r, Postgres> + Type<Postgres>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    all(feature = "postgres")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Postgres> + Type<Postgres>
{}

#[cfg(all(
    not(any(feature = "mysql")),
    all(feature = "postgres", feature = "sqlite")
))]
pub trait AnyDecode<'r>:
Decode<'r, Postgres> + Type<Postgres> + Decode<'r, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "mysql")),
    all(feature = "postgres", feature = "sqlite")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Postgres> + Type<Postgres> + Decode<'r, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    all(feature = "mysql")
))]
pub trait AnyDecode<'r>: Decode<'r, MySql> + Type<MySql> {}


#[cfg(all(
    not(any(feature = "postgres", feature = "mysql")),
    all(feature = "sqlite")
))]
pub trait AnyDecode<'r>: Decode<'r, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "mysql")),
    all(feature = "sqlite")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres")),
    all(feature = "mysql", feature = "sqlite")
))]
pub trait AnyDecode<'r>:
Decode<'r, MySql> + Type<MySql> + Decode<'r, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres")),
    all(feature = "mysql", feature = "sqlite")
))]
impl<'r, T> AnyDecode<'r> for T where
    T: Decode<'r, MySql> + Type<MySql> + Decode<'r, Sqlite> + Type<Sqlite>
{}

// only 1 (4)

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    feature = "postgres"
))]
pub trait AnyDecode<'r>: Decode<'r, Postgres> + Type<Postgres> {}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    feature = "postgres"
))]
impl<'r, T> AnyDecode<'r> for T where T: Decode<'r, Postgres> + Type<Postgres> {}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    feature = "mysql"
))]
impl<'r, T> AnyDecode<'r> for T where T: Decode<'r, MySql> + Type<MySql> {}

#[cfg(all(
    not(any(feature = "mysql", feature = "postgres")),
    feature = "sqlite"
))]
pub trait AnyDecode<'r>: Decode<'r, Sqlite> + Type<Sqlite> {}

#[cfg(all(
    not(any(feature = "mysql", feature = "postgres")),
    feature = "sqlite"
))]
impl<'r, T> AnyDecode<'r> for T where T: Decode<'r, Sqlite> + Type<Sqlite> {}
