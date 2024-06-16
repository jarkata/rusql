use crate::encode::Encode;
use crate::types::Type;

#[cfg(feature = "postgres")]
use crate::postgres::Postgres;

#[cfg(feature = "mysql")]
use crate::mysql::MySql;

#[cfg(feature = "sqlite")]
use crate::sqlite::Sqlite;

// Implements Encode for any T where T supports Encode for any database that has support currently
// compiled into RuSQL
macro_rules! impl_any_encode {
    ($ty:ty) => {
        impl<'q> crate::encode::Encode<'q, crate::any::Any> for $ty
        where
            $ty: crate::any::AnyEncode<'q>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut crate::any::AnyArgumentBuffer<'q>,
            ) -> crate::encode::IsNull {
                match &mut buf.0 {
                    #[cfg(feature = "postgres")]
                    crate::any::arguments::AnyArgumentBufferKind::Postgres(args, _) => {
                        args.add(self)
                    }

                    #[cfg(feature = "mysql")]
                    crate::any::arguments::AnyArgumentBufferKind::MySql(args, _) => args.add(self),

                    #[cfg(feature = "sqlite")]
                    crate::any::arguments::AnyArgumentBufferKind::Sqlite(args) => args.add(self),
                }

                // unused
                crate::encode::IsNull::No
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
pub trait AnyEncode<'q>:
Encode<'q, Postgres>
+ Type<Postgres>
+ Encode<'q, MySql>
+ Type<MySql>
+ Encode<'q, Sqlite>
+ Type<Sqlite>
{}

#[cfg(all(
    feature = "postgres",
    feature = "mysql",
    feature = "sqlite"
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres>
    + Type<Postgres>
    + Encode<'q, MySql>
    + Type<MySql>
    + Encode<'q, Sqlite>
    + Type<Sqlite>
{}

#[cfg(all(
    all(feature = "postgres", feature = "mysql", feature = "sqlite")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres>
    + Type<Postgres>
    + Encode<'q, MySql>
    + Type<MySql>
    + Encode<'q, Sqlite>
    + Type<Sqlite>
{}

#[cfg(all(
    not(feature = "mysql"),
    all(feature = "postgres", feature = "sqlite")
))]
pub trait AnyEncode<'q>:
Encode<'q, Postgres>
+ Type<Postgres>
+ Encode<'q, Sqlite>
+ Type<Sqlite>
{}

#[cfg(all(
    not(feature = "mysql"),
    all(feature = "postgres", feature = "sqlite")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres>
    + Type<Postgres>
    + Encode<'q, Sqlite>
    + Type<Sqlite>
{}

#[cfg(all(
    not(feature = "sqlite"),
    all(feature = "postgres", feature = "mysql")
))]
pub trait AnyEncode<'q>:
Encode<'q, Postgres>
+ Type<Postgres>
+ Encode<'q, MySql>
+ Type<MySql>
{}

#[cfg(all(
    not(feature = "sqlite"),
    all(feature = "postgres", feature = "mysql")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres>
    + Type<Postgres>
    + Encode<'q, MySql>
    + Type<MySql>
{}

#[cfg(all(
    not(feature = "postgres"),
    all(feature = "sqlite", feature = "mysql")
))]
pub trait AnyEncode<'q>:
Encode<'q, Sqlite>
+ Type<Sqlite>
+ Encode<'q, MySql>
+ Type<MySql>
{}

#[cfg(all(
    not(feature = "postgres"),
    all(feature = "sqlite", feature = "mysql")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Sqlite>
    + Type<Sqlite>
    + Encode<'q, MySql>
    + Type<MySql>
{}

// only 2 (6)

#[cfg(all(
    not(any(feature = "sqlite")),
    all(feature = "postgres", feature = "mysql")
))]
pub trait AnyEncode<'q>:
Encode<'q, Postgres> + Type<Postgres> + Encode<'q, MySql> + Type<MySql>
{}

#[cfg(all(
    not(any(feature = "sqlite")),
    all(feature = "postgres", feature = "mysql")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres> + Type<Postgres> + Encode<'q, MySql> + Type<MySql>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    all(feature = "postgres")
))]
pub trait AnyEncode<'q>:
Encode<'q, Postgres> + Type<Postgres>
{}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    all(feature = "postgres")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres> + Type<Postgres>
{}

#[cfg(all(
    not(any(feature = "mysql")),
    all(feature = "postgres", feature = "sqlite")
))]
pub trait AnyEncode<'q>:
Encode<'q, Postgres> + Type<Postgres> + Encode<'q, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "mysql")),
    all(feature = "postgres", feature = "sqlite")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Postgres> + Type<Postgres> + Encode<'q, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    all(feature = "mysql")
))]
pub trait AnyEncode<'q>: Encode<'q, MySql> + Type<MySql> {}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    all(feature = "mysql")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, MySql> + Type<MySql>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "mysql")),
    all(feature = "sqlite")
))]
pub trait AnyEncode<'q>: Encode<'q, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres", feature = "mysql")),
    all(feature = "sqlite")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres")),
    all(feature = "mysql", feature = "sqlite")
))]
pub trait AnyEncode<'q>:
Encode<'q, MySql> + Type<MySql> + Encode<'q, Sqlite> + Type<Sqlite>
{}

#[cfg(all(
    not(any(feature = "postgres")),
    all(feature = "mysql", feature = "sqlite")
))]
impl<'q, T> AnyEncode<'q> for T where
    T: Encode<'q, MySql> + Type<MySql> + Encode<'q, Sqlite> + Type<Sqlite>
{}

// only 1 (4)

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    feature = "postgres"
))]
pub trait AnyEncode<'q>: Encode<'q, Postgres> + Type<Postgres> {}

#[cfg(all(
    not(any(feature = "mysql", feature = "sqlite")),
    feature = "postgres"
))]
impl<'q, T> AnyEncode<'q> for T where T: Encode<'q, Postgres> + Type<Postgres> {}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    feature = "mysql"
))]
pub trait AnyEncode<'q>: Encode<'q, MySql> + Type<MySql> {}

#[cfg(all(
    not(any(feature = "postgres", feature = "sqlite")),
    feature = "mysql"
))]
impl<'q, T> AnyEncode<'q> for T where T: Encode<'q, MySql> + Type<MySql> {}

#[cfg(all(
    not(any(feature = "mysql", feature = "postgres")),
    feature = "sqlite"
))]
pub trait AnyEncode<'q>: Encode<'q, Sqlite> + Type<Sqlite> {}

#[cfg(all(
    not(any(feature = "mysql", feature = "postgres")),
    feature = "sqlite"
))]
impl<'q, T> AnyEncode<'q> for T where T: Encode<'q, Sqlite> + Type<Sqlite> {}
