/// Statically checked SQL query with `println!()` style syntax.
///
/// This expands to an instance of [`query::Map`][crate::query::Map] that outputs an ad-hoc anonymous
/// struct type, if the query has at least one output column that is not `Void`, or `()` (unit) otherwise:
///
/// ```rust,ignore
/// # use sqlx::Connect;
/// # #[cfg(all(feature = "mysql", feature = "_rt-async-std"))]
/// # #[async_std::main]
/// # async fn main() -> sqlx::Result<()>{
/// # let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
/// #
/// # if !(db_url.starts_with("mysql") || db_url.starts_with("mariadb")) { return Ok(()) }
/// # let mut conn = sqlx::MySqlConnection::connect(db_url).await?;
/// // let mut conn = <impl sqlx::Executor>;
/// let account = sqlx::query!("select (1) as id, 'Herp Derpinson' as name")
///     .fetch_one(&mut conn)
///     .await?;
///
/// // anonymous struct has `#[derive(Debug)]` for convenience
/// println!("{:?}", account);
/// println!("{}: {}", account.id, account.name);
///
/// # Ok(())
/// # }
/// #
/// # #[cfg(any(not(feature = "mysql"), not(feature = "_rt-async-std")))]
/// # fn main() {}
/// ```
///
/// **The method you want to call depends on how many rows you're expecting.**
///
/// | Number of Rows | Method to Call*             | Returns                                             | Notes |
/// |----------------| ----------------------------|-----------------------------------------------------|-------|
/// | None†          | `.execute(...).await`       | `sqlx::Result<DB::QueryResult>`                     | For `INSERT`/`UPDATE`/`DELETE` without `RETURNING`. |
/// | Zero or One    | `.fetch_optional(...).await`| `sqlx::Result<Option<{adhoc struct}>>`              | Extra rows are ignored. |
/// | Exactly One    | `.fetch_one(...).await`     | `sqlx::Result<{adhoc struct}>`                      | Errors if no rows were returned. Extra rows are ignored. Aggregate queries, use this. |
/// | At Least One   | `.fetch(...)`               | `impl Stream<Item = sqlx::Result<{adhoc struct}>>`  | Call `.try_next().await` to get each row result. |
/// | Multiple   | `.fetch_all(...)`               | `sqlx::Result<Vec<{adhoc struct}>>`  | |
///
/// \* All methods accept one of `&mut {connection type}`, `&mut Transaction` or `&Pool`.
/// † Only callable if the query returns no columns; otherwise it's assumed the query *may* return at least one row.
/// ## Requirements
/// * The `DATABASE_URL` environment variable must be set at build-time to point to a database
/// server with the schema that the query string will be checked against. All variants of `query!()`
/// use [dotenv]<sup>1</sup> so this can be in a `.env` file instead.
///
///     * Or, `sqlx-data.json` must exist at the workspace root. See [Offline Mode](#offline-mode-requires-the-offline-feature)
///       below.
///
/// * The query must be a string literal, or concatenation of string literals using `+` (useful
/// for queries generated by macro), or else it cannot be introspected (and thus cannot be dynamic
/// or the result of another macro).
///
/// * The `QueryAs` instance will be bound to the same database type as `query!()` was compiled
/// against (e.g. you cannot build against a Postgres database and then run the query against
/// a MySQL database).
///
///     * The schema of the database URL (e.g. `postgres://` or `mysql://`) will be used to
///       determine the database type.
///
/// <sup>1</sup> The `dotenv` crate itself appears abandoned as of [December 2021](https://github.com/dotenv-rs/dotenv/issues/74)
/// so we now use the [`dotenvy`] crate instead. The file format is the same.
///
/// [dotenv]: https://crates.io/crates/dotenv
/// [dotenvy]: https://crates.io/crates/dotenvy
/// ## Query Arguments
/// Like `println!()` and the other formatting macros, you can add bind parameters to your SQL
/// and this macro will typecheck passed arguments and error on missing ones:
///
/// ```rust,ignore
/// # use sqlx::Connect;
/// # #[cfg(all(feature = "mysql", feature = "_rt-async-std"))]
/// # #[async_std::main]
/// # async fn main() -> sqlx::Result<()>{
/// # let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
/// #
/// # if !(db_url.starts_with("mysql") || db_url.starts_with("mariadb")) { return Ok(()) }
/// # let mut conn = sqlx::mysql::MySqlConnection::connect(db_url).await?;
/// // let mut conn = <impl sqlx::Executor>;
/// let account = sqlx::query!(
///         // just pretend "accounts" is a real table
///         "select * from (select (1) as id, 'Herp Derpinson' as name) accounts where id = ?",
///         1i32
///     )
///     .fetch_one(&mut conn)
///     .await?;
///
/// println!("{:?}", account);
/// println!("{}: {}", account.id, account.name);
/// # Ok(())
/// # }
/// #
/// # #[cfg(any(not(feature = "mysql"), not(feature = "_rt-async-std")))]
/// # fn main() {}
/// ```
///
/// Bind parameters in the SQL string are specific to the database backend:
///
/// * Postgres: `$N` where `N` is the 1-based positional argument index
/// * MySQL/SQLite: `?` which matches arguments in order that it appears in the query
///
/// ## Nullability: Bind Parameters
/// For a given expected type `T`, both `T` and `Option<T>` are allowed (as well as either
/// behind references). `Option::None` will be bound as `NULL`, so if binding a type behind `Option`
/// be sure your query can support it.
///
/// Note, however, if binding in a `where` clause, that equality comparisons with `NULL` may not
/// work as expected; instead you must use `IS NOT NULL` or `IS NULL` to check if a column is not
/// null or is null, respectively.
///
/// In Postgres and MySQL you may also use `IS [NOT] DISTINCT FROM` to compare with a possibly
/// `NULL` value. In MySQL `IS NOT DISTINCT FROM` can be shortened to `<=>`.
/// In SQLite you can use `IS` or `IS NOT`. Note that operator precedence may be different.
///
/// ## Nullability: Output Columns
/// In most cases, the database engine can tell us whether or not a column may be `NULL`, and
/// the `query!()` macro adjusts the field types of the returned struct accordingly.
///
/// For Postgres, this only works for columns which come directly from actual tables,
/// as the implementation will need to query the table metadata to find if a given column
/// has a `NOT NULL` constraint. Columns that do not have a `NOT NULL` constraint or are the result
/// of an expression are assumed to be nullable and so `Option<T>` is used instead of `T`.
///
/// For MySQL, the implementation looks at [the `NOT_NULL` flag](https://dev.mysql.com/doc/dev/mysql-server/8.0.12/group__group__cs__column__definition__flags.html#ga50377f5ca5b3e92f3931a81fe7b44043)
/// of [the `ColumnDefinition` structure in `COM_QUERY_OK`](https://dev.mysql.com/doc/internals/en/com-query-response.html#column-definition):
/// if it is set, `T` is used; if it is not set, `Option<T>` is used.
///
/// MySQL appears to be capable of determining the nullability of a result column even if it
/// is the result of an expression, depending on if the expression may in any case result in
/// `NULL` which then depends on the semantics of what functions are used. Consult the MySQL
/// manual for the functions you are using to find the cases in which they return `NULL`.
///
/// For SQLite we perform a similar check to Postgres, looking for `NOT NULL` constraints
/// on columns that come from tables. However, for SQLite we also can step through the output
/// of `EXPLAIN` to identify columns that may or may not be `NULL`.
///
/// To override the nullability of an output column, [see below](#type-overrides-output-columns).
///
/// ## Type Overrides: Bind Parameters (Postgres only)
/// For typechecking of bind parameters, casts using `as` are treated as overrides for the inferred
/// types of bind parameters and no typechecking is emitted:
///
/// ```rust,ignore
/// #[derive(sqlx::Type)]
/// #[sqlx(transparent)]
/// struct MyInt4(i32);
///
/// let my_int = MyInt4(1);
///
/// sqlx::query!("select $1::int4 as id", my_int as MyInt4)
/// ```
///
/// Using `expr as _` or `expr : _` simply signals to the macro to not type-check that bind expression,
/// and then that syntax is stripped from the expression so as to not trigger type errors
/// (or an unstable syntax feature in the case of the latter, which is called type ascription).
///
/// ## Type Overrides: Output Columns
/// Type overrides are also available for output columns, utilizing the SQL standard's support
/// for arbitrary text in column names:
///
/// ##### Force Not-Null
/// Selecting a column `foo as "foo!"` (Postgres / SQLite) or `` foo as `foo!` `` (MySQL) overrides
/// inferred nullability and forces the column to be treated as `NOT NULL`; this is useful e.g. for
/// selecting expressions in Postgres where we cannot infer nullability:
///
/// ```rust,ignore
/// # async fn main() {
/// # let mut conn = panic!();
/// // Postgres: using a raw query string lets us use unescaped double-quotes
/// // Note that this query wouldn't work in SQLite as we still don't know the exact type of `id`
/// let record = sqlx::query!(r#"select 1 as "id!""#) // MySQL: use "select 1 as `id!`" instead
///     .fetch_one(&mut conn)
///     .await?;
///
/// // For Postgres this would have been inferred to be Option<i32> instead
/// assert_eq!(record.id, 1i32);
/// # }
///
/// ```
///
/// ##### Force Nullable
/// Selecting a column `foo as "foo?"` (Postgres / SQLite) or `` foo as `foo?` `` (MySQL) overrides
/// inferred nullability and forces the column to be treated as nullable; this is provided mainly
/// for symmetry with `!`.
///
/// ```rust,ignore
/// # async fn main() {
/// # let mut conn = panic!();
/// // Postgres/SQLite:
/// let record = sqlx::query!(r#"select 1 as "id?""#) // MySQL: use "select 1 as `id?`" instead
///     .fetch_one(&mut conn)
///     .await?;
///
/// // For Postgres this would have been inferred to be Option<i32> anyway
/// // but this is just a basic example
/// assert_eq!(record.id, Some(1i32));
/// # }
/// ```
///
/// MySQL should be accurate with regards to nullability as it directly tells us when a column is
/// expected to never be `NULL`. Any mistakes should be considered a bug in MySQL.
///
/// However, inference in SQLite and Postgres is more fragile as it depends primarily on observing
/// `NOT NULL` constraints on columns. If a `NOT NULL` column is brought in by a `LEFT JOIN` then
/// that column may be `NULL` if its row does not satisfy the join condition. Similarly, a
/// `FULL JOIN` or `RIGHT JOIN` may generate rows from the primary table that are all `NULL`.
///
/// Unfortunately, the result of mistakes in inference is a `UnexpectedNull` error at runtime.
///
/// In Postgres, we patch up this inference by analyzing `EXPLAIN VERBOSE` output (which is not
/// well documented, is highly dependent on the query plan that Postgres generates, and may differ
/// between releases) to find columns that are the result of left/right/full outer joins. This
/// analysis errs on the side of producing false positives (marking columns nullable that are not
/// in practice) but there are likely edge cases that it does not cover yet.
///
/// Using `?` as an override we can fix this for columns we know to be nullable in practice:
///
/// ```rust,ignore
/// # async fn main() {
/// # let mut conn = panic!();
/// // Ironically this is the exact column we primarily look at to determine nullability in Postgres
/// let record = sqlx::query!(
///     r#"select attnotnull as "attnotnull?" from (values (1)) ids left join pg_attribute on false"#
/// )
/// .fetch_one(&mut conn)
/// .await?;
///
/// // Although we do our best, under Postgres this might have been inferred to be `bool`
/// // In that case, we would have gotten an error
/// assert_eq!(record.attnotnull, None);
/// # }
/// ```
///
/// If you find that you need to use this override, please open an issue with a query we can use
/// to reproduce the problem. For Postgres users, especially helpful would be the output of
/// `EXPLAIN (VERBOSE, FORMAT JSON) <your query>` with bind parameters substituted in the query
/// (as the exact value of bind parameters can change the query plan)
/// and the definitions of any relevant tables (or sufficiently anonymized equivalents).
///
/// ##### Force a Different/Custom Type
/// Selecting a column `foo as "foo: T"` (Postgres / SQLite) or `` foo as `foo: T` `` (MySQL)
/// overrides the inferred type which is useful when selecting user-defined custom types
/// (dynamic type checking is still done so if the types are incompatible this will be an error
/// at runtime instead of compile-time). Note that this syntax alone doesn't override inferred nullability,
/// but it is compatible with the forced not-null and forced nullable annotations:
///
/// ```rust,ignore
/// # async fn main() {
/// # let mut conn = panic!();
/// #[derive(sqlx::Type)]
/// #[sqlx(transparent)]
/// struct MyInt4(i32);
///
/// let my_int = MyInt4(1);
///
/// // Postgres/SQLite
/// sqlx::query!(r#"select 1 as "id!: MyInt4""#) // MySQL: use "select 1 as `id: MyInt4`" instead
///     .fetch_one(&mut conn)
///     .await?;
///
/// // For Postgres this would have been inferred to be `Option<i32>`, MySQL/SQLite `i32`
/// // Note that while using `id: MyInt4` (without the `!`) would work the same for MySQL/SQLite,
/// // Postgres would expect `Some(MyInt4(1))` and the code wouldn't compile
/// assert_eq!(record.id, MyInt4(1));
/// # }
/// ```
///
/// ##### Overrides cheatsheet
///
/// | Syntax    | Nullability     | Type       |
/// | --------- | --------------- | ---------- |
/// | `foo!`    | Forced not-null | Inferred   |
/// | `foo?`    | Forced nullable | Inferred   |
/// | `foo: T`  | Inferred        | Overridden |
/// | `foo!: T` | Forced not-null | Overridden |
/// | `foo?: T` | Forced nullable | Overridden |
///
/// ## Offline Mode (requires the `offline` feature)
/// The macros can be configured to not require a live database connection for compilation,
/// but it requires a couple extra steps:
///
/// * Run `cargo install sqlx-cli`.
/// * In your project with `DATABASE_URL` set (or in a `.env` file) and the database server running,
///   run `cargo sqlx prepare`.
/// * Check the generated `sqlx-data.json` file into version control.
/// * Don't have `DATABASE_URL` set during compilation.
///
/// Your project can now be built without a database connection (you must omit `DATABASE_URL` or
/// else it will still try to connect). To update the generated file simply run `cargo sqlx prepare`
/// again.
///
/// To ensure that your `sqlx-data.json` file is kept up-to-date, both with the queries in your
/// project and your database schema itself, run
/// `cargo install sqlx-cli && cargo sqlx prepare --check` in your Continuous Integration script.
///
/// See [the README for `sqlx-cli`](https://crates.io/crates/sqlx-cli) for more information.
///
/// ## See Also
/// * [query_as!] if you want to use a struct you can name,
/// * [query_file!] if you want to define the SQL query out-of-line,
/// * [query_file_as!] if you want both of the above.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query (
    // in Rust 1.45 we can now invoke proc macros in expression position
    ($query:expr) => ({
        $crate::sqlx_macros::expand_query!(source = $query)
    });
    // RFC: this semantically should be `$($args:expr),*` (with `$(,)?` to allow trailing comma)
    // but that doesn't work in 1.45 because `expr` fragments get wrapped in a way that changes
    // their hygiene, which is fixed in 1.46 so this is technically just a temp. workaround.
    // My question is: do we care?
    // I was hoping using the `expr` fragment might aid code completion but it doesn't in my
    // experience, at least not with IntelliJ-Rust at the time of writing (version 0.3.126.3220-201)
    // so really the only benefit is making the macros _slightly_ self-documenting, but it's
    // not like it makes them magically understandable at-a-glance.
    ($query:expr, $($args:tt)*) => ({
        $crate::sqlx_macros::expand_query!(source = $query, args = [$($args)*])
    })
);

/// A variant of [query!] which does not check the input or output types. This still does parse
/// the query to ensure it's syntactically and semantically valid for the current database.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_unchecked (
    ($query:expr) => ({
        $crate::sqlx_macros::expand_query!(source = $query, checked = false)
    });
    ($query:expr, $($args:tt)*) => ({
        $crate::sqlx_macros::expand_query!(source = $query, args = [$($args)*], checked = false)
    })
);

/// A variant of [query!] where the SQL query is stored in a separate file.
///
/// Useful for large queries and potentially cleaner than multiline strings.
///
/// The syntax and requirements (see [query!]) are the same except the SQL string is replaced by a
/// file path.
///
/// The file must be relative to the project root (the directory containing `Cargo.toml`),
/// unlike `include_str!()` which uses compiler internals to get the path of the file where it
/// was invoked.
///
/// -----
///
/// `examples/queries/account-by-id.sql`:
/// ```text
/// select * from (select (1) as id, 'Herp Derpinson' as name) accounts
/// where id = ?
/// ```
///
/// `src/my_query.rs`:
/// ```rust,ignore
/// # use sqlx::Connect;
/// # #[cfg(all(feature = "mysql", feature = "_rt-async-std"))]
/// # #[async_std::main]
/// # async fn main() -> sqlx::Result<()>{
/// # let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
/// #
/// # if !(db_url.starts_with("mysql") || db_url.starts_with("mariadb")) { return Ok(()) }
/// # let mut conn = sqlx::MySqlConnection::connect(db_url).await?;
/// let account = sqlx::query_file!("tests/test-query-account-by-id.sql", 1i32)
///     .fetch_one(&mut conn)
///     .await?;
///
/// println!("{:?}", account);
/// println!("{}: {}", account.id, account.name);
///
/// # Ok(())
/// # }
/// #
/// # #[cfg(any(not(feature = "mysql"), not(feature = "_rt-async-std")))]
/// # fn main() {}
/// ```
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_file (
    ($path:literal) => ({
        $crate::sqlx_macros::expand_query!(source_file = $path)
    });
    ($path:literal, $($args:tt)*) => ({
        $crate::sqlx_macros::expand_query!(source_file = $path, args = [$($args)*])
    })
);

/// A variant of [query_file!] which does not check the input or output types. This still does parse
/// the query to ensure it's syntactically and semantically valid for the current database.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_file_unchecked (
    ($path:literal) => ({
        $crate::sqlx_macros::expand_query!(source_file = $path, checked = false)
    });
    ($path:literal, $($args:tt)*) => ({
        $crate::sqlx_macros::expand_query!(source_file = $path, args = [$($args)*], checked = false)
    })
);

/// A variant of [query!] which takes a path to an explicitly defined struct as the output type.
///
/// This lets you return the struct from a function or add your own trait implementations.
///
/// **This macro does not use [`FromRow`][crate::FromRow]**; in fact, no trait implementations are
/// required at all, though this may change in future versions.
///
/// The macro maps rows using a struct literal where the names of columns in the query are expected
/// to be the same as the fields of the struct (but the order does not need to be the same).
/// The types of the columns are based on the query and not the corresponding fields of the struct,
/// so this is type-safe as well.
///
/// This enforces a few things:
/// * The query must output at least one column.
/// * The column names of the query must match the field names of the struct.
/// * The field types must be the Rust equivalent of their SQL counterparts; see the corresponding
/// module for your database for mappings:
///     * Postgres: [crate::postgres::types]
///     * MySQL: [crate::mysql::types]
///     * SQLite: [crate::sqlite::types]
///     * MSSQL: [crate::mssql::types]
/// * If a column may be `NULL`, the corresponding field's type must be wrapped in `Option<_>`.
/// * Neither the query nor the struct may have unused fields.
///
/// The only modification to the `query!()` syntax is that the struct name is given before the SQL
/// string:
/// ```rust,ignore
/// # use sqlx::Connect;
/// # #[cfg(all(feature = "mysql", feature = "_rt-async-std"))]
/// # #[async_std::main]
/// # async fn main() -> sqlx::Result<()>{
/// # let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
/// #
/// # if !(db_url.starts_with("mysql") || db_url.starts_with("mariadb")) { return Ok(()) }
/// # let mut conn = sqlx::MySqlConnection::connect(db_url).await?;
/// #[derive(Debug)]
/// struct Account {
///     id: i32,
///     name: String
/// }
///
/// // let mut conn = <impl sqlx::Executor>;
/// let account = sqlx::query_as!(
///         Account,
///         "select * from (select (1) as id, 'Herp Derpinson' as name) accounts where id = ?",
///         1i32
///     )
///     .fetch_one(&mut conn)
///     .await?;
///
/// println!("{:?}", account);
/// println!("{}: {}", account.id, account.name);
///
/// # Ok(())
/// # }
/// #
/// # #[cfg(any(not(feature = "mysql"), not(feature = "_rt-async-std")))]
/// # fn main() {}
/// ```
///
/// **The method you want to call depends on how many rows you're expecting.**
///
/// | Number of Rows | Method to Call*             | Returns (`T` being the given struct)   | Notes |
/// |----------------| ----------------------------|----------------------------------------|-------|
/// | Zero or One    | `.fetch_optional(...).await`| `sqlx::Result<Option<T>>`              | Extra rows are ignored. |
/// | Exactly One    | `.fetch_one(...).await`     | `sqlx::Result<T>`                      | Errors if no rows were returned. Extra rows are ignored. Aggregate queries, use this. |
/// | At Least One   | `.fetch(...)`               | `impl Stream<Item = sqlx::Result<T>>`  | Call `.try_next().await` to get each row result. |
/// | Multiple       | `.fetch_all(...)`           | `sqlx::Result<Vec<T>>`  | |
///
/// \* All methods accept one of `&mut {connection type}`, `&mut Transaction` or `&Pool`.
/// (`.execute()` is omitted as this macro requires at least one column to be returned.)
///
/// ### Column Type Override: Infer from Struct Field
/// In addition to the column type overrides supported by [query!], `query_as!()` supports an
/// additional override option:
///
/// If you select a column `foo as "foo: _"` (Postgres/SQLite) or `` foo as `foo: _` `` (MySQL)
/// it causes that column to be inferred based on the type of the corresponding field in the given
/// record struct. Runtime type-checking is still done so an error will be emitted if the types
/// are not compatible.
///
/// This allows you to override the inferred type of a column to instead use a custom-defined type:
///
/// ```rust,ignore
/// #[derive(sqlx::Type)]
/// #[sqlx(transparent)]
/// struct MyInt4(i32);
///
/// struct Record {
///     id: MyInt4,
/// }
///
/// let my_int = MyInt4(1);
///
/// // Postgres/SQLite
/// sqlx::query_as!(Record, r#"select 1 as "id: _""#) // MySQL: use "select 1 as `id: _`" instead
///     .fetch_one(&mut conn)
///     .await?;
///
/// assert_eq!(record.id, MyInt4(1));
/// ```
///
/// ### Troubleshooting: "error: mismatched types"
/// If you get a "mismatched types" error from an invocation of this macro and the error
/// isn't pointing specifically at a parameter.
///
/// For example, code like this (using a Postgres database):
///
/// ```rust,ignore
/// struct Account {
///     id: i32,
///     name: Option<String>,
/// }
///
/// let account = sqlx::query_as!(
///     Account,
///     r#"SELECT id, name from (VALUES (1, 'Herp Derpinson')) accounts(id, name)"#,
/// )
///     .fetch_one(&mut conn)
///     .await?;
/// ```
///
/// Might produce an error like this:
/// ```text,ignore
/// error[E0308]: mismatched types
///    --> tests/postgres/macros.rs:126:19
///     |
/// 126 |       let account = sqlx::query_as!(
///     |  ___________________^
/// 127 | |         Account,
/// 128 | |         r#"SELECT id, name from (VALUES (1, 'Herp Derpinson')) accounts(id, name)"#,
/// 129 | |     )
///     | |_____^ expected `i32`, found enum `std::option::Option`
///     |
///     = note: expected type `i32`
///                found enum `std::option::Option<i32>`
/// ```
///
/// This means that you need to check that any field of the "expected" type (here, `i32`) matches
/// the Rust type mapping for its corresponding SQL column (see the `types` module of your database,
/// listed above, for mappings). The "found" type is the SQL->Rust mapping that the macro chose.
///
/// In the above example, the returned column is inferred to be nullable because it's being
/// returned from a `VALUES` statement in Postgres, so the macro inferred the field to be nullable
/// and so used `Option<i32>` instead of `i32`. **In this specific case** we could use
/// `select id as "id!"` to override the inferred nullability because we know in practice
/// that column will never be `NULL` and it will fix the error.
///
/// Nullability inference and type overrides are discussed in detail in the docs for [query!].
///
/// It unfortunately doesn't appear to be possible right now to make the error specifically mention
/// the field; this probably requires the `const-panic` feature (still unstable as of Rust 1.45).
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_as (
    ($out_struct:path, $query:expr) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source = $query)
    });
    ($out_struct:path, $query:expr, $($args:tt)*) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source = $query, args = [$($args)*])
    })
);

/// Combines the syntaxes of [query_as!] and [query_file!].
///
/// Enforces requirements of both macros; see them for details.
///
/// ```rust,ignore
/// # use sqlx::Connect;
/// # #[cfg(all(feature = "mysql", feature = "_rt-async-std"))]
/// # #[async_std::main]
/// # async fn main() -> sqlx::Result<()>{
/// # let db_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
/// #
/// # if !(db_url.starts_with("mysql") || db_url.starts_with("mariadb")) { return Ok(()) }
/// # let mut conn = sqlx::MySqlConnection::connect(db_url).await?;
/// #[derive(Debug)]
/// struct Account {
///     id: i32,
///     name: String
/// }
///
/// // let mut conn = <impl sqlx::Executor>;
/// let account = sqlx::query_file_as!(Account, "tests/test-query-account-by-id.sql", 1i32)
///     .fetch_one(&mut conn)
///     .await?;
///
/// println!("{:?}", account);
/// println!("{}: {}", account.id, account.name);
///
/// # Ok(())
/// # }
/// #
/// # #[cfg(any(not(feature = "mysql"), not(feature = "_rt-async-std")))]
/// # fn main() {}
/// ```
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_file_as (
    ($out_struct:path, $path:literal) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source_file = $path)
    });
    ($out_struct:path, $path:literal, $($args:tt)*) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source_file = $path, args = [$($args)*])
    })
);

/// A variant of [query_as!] which does not check the input or output types. This still does parse
/// the query to ensure it's syntactically and semantically valid for the current database.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_as_unchecked (
    ($out_struct:path, $query:expr) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source = $query, checked = false)
    });

    ($out_struct:path, $query:expr, $($args:tt)*) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source = $query, args = [$($args)*], checked = false)
    })
);

/// A variant of [query_file_as!] which does not check the input or output types. This
/// still does parse the query to ensure it's syntactically and semantically valid
/// for the current database.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_file_as_unchecked (
    ($out_struct:path, $path:literal) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source_file = $path, checked = false)
    });

    ($out_struct:path, $path:literal, $($args:tt)*) => ( {
        $crate::sqlx_macros::expand_query!(record = $out_struct, source_file = $path, args = [$($args)*], checked = false)
    })
);

/// A variant of [query!] which expects a single column from the query and evaluates to an
/// instance of [QueryScalar][crate::query::QueryScalar].
///
/// The name of the column is not required to be a valid Rust identifier, however you can still
/// use the column type override syntax in which case the column name _does_ have to be a valid
/// Rust identifier for the override to parse properly. If the override parse fails the error
/// is silently ignored (we just don't have a reliable way to tell the difference). **If you're
/// getting a different type than expected, please check to see if your override syntax is correct
/// before opening an issue.**
///
/// Wildcard overrides like in [query_as!] are also allowed, in which case the output type
/// is left up to inference.
///
/// See [query!] for more information.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_scalar (
    ($query:expr) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source = $query)
    );
    ($query:expr, $($args:tt)*) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source = $query, args = [$($args)*])
    )
);

/// A variant of [query_scalar!] which takes a file path like [query_file!].
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_file_scalar (
    ($path:literal) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source_file = $path)
    );
    ($path:literal, $($args:tt)*) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source_file = $path, args = [$($args)*])
    )
);

/// A variant of [query_scalar!] which does not typecheck bind parameters and leaves the output type
/// to inference. The query itself is still checked that it is syntactically and semantically
/// valid for the database, that it only produces one column and that the number of bind parameters
/// is correct.
///
/// For this macro variant the name of the column is irrelevant.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_scalar_unchecked (
    ($query:expr) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source = $query, checked = false)
    );
    ($query:expr, $($args:tt)*) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source = $query, args = [$($args)*], checked = false)
    )
);

/// A variant of [query_file_scalar!] which does not typecheck bind parameters and leaves the output
/// type to inference. The query itself is still checked that it is syntactically and
/// semantically valid for the database, that it only produces one column and that the number of
/// bind parameters is correct.
///
/// For this macro variant the name of the column is irrelevant.
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
macro_rules! query_file_scalar_unchecked (
    ($path:literal) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source_file = $path, checked = false)
    );
    ($path:literal, $($args:tt)*) => (
        $crate::sqlx_macros::expand_query!(scalar = _, source_file = $path, args = [$($args)*], checked = false)
    )
);

/// Embeds migrations into the binary by expanding to a static instance of [Migrator][crate::migrate::Migrator].
///
/// ```rust,ignore
/// sqlx::migrate!("db/migrations")
///     .run(&pool)
///     .await?;
/// ```
///
/// ```rust,ignore
/// use sqlx::migrate::Migrator;
///
/// static MIGRATOR: Migrator = sqlx::migrate!(); // defaults to "./migrations"
/// ```
///
/// The directory must be relative to the project root (the directory containing `Cargo.toml`),
/// unlike `include_str!()` which uses compiler internals to get the path of the file where it
/// was invoked.
///
/// See [MigrationSource][crate::migrate::MigrationSource] for details on structure of the ./migrations directory.
///
/// ## Triggering Recompilation on Migration Changes
/// In some cases when making changes to embedded migrations, such as adding a new migration without
/// changing any Rust source files, you might find that `cargo build` doesn't actually do anything,
/// or when you do `cargo run` your application isn't applying new migrations on startup.
///
/// This is because our ability to tell the compiler to watch external files for changes
/// from a proc-macro is very limited. The compiler by default only re-runs proc macros when
/// one ore more source files have changed, because normally it shouldn't have to otherwise. SQLx is
/// just weird in that external factors can change the output of proc macros, much to the chagrin of
/// the compiler team and IDE plugin authors.
///
/// As of 0.5.6, we emit `include_str!()` with an absolute path for each migration, but that
/// only works to get the compiler to watch _existing_ migration files for changes.
///
/// Our only options for telling it to watch the whole `migrations/` directory are either via the
/// user creating a Cargo build script in their project, or using an unstable API on nightly
/// governed by a `cfg`-flag.
///
/// ##### Stable Rust: Cargo Build Script
/// The only solution on stable Rust right now is to create a Cargo build script in your project
/// and have it print `cargo:rerun-if-changed=migrations`:
///
/// `build.rs`
/// ```
/// fn main() {
///     println!("cargo:rerun-if-changed=migrations");
/// }
/// ```
///
/// You can run `sqlx migrate build-script` to generate this file automatically.
///
/// See: [The Cargo Book: 3.8 Build Scripts; Outputs of the Build Script](https://doc.rust-lang.org/stable/cargo/reference/build-scripts.html#outputs-of-the-build-script)
///
/// #### Nightly Rust: `cfg` Flag
/// The `migrate!()` macro also listens to `--cfg sqlx_macros_unstable`, which will enable
/// the `track_path` feature to directly tell the compiler to watch the `migrations/` directory:
///
/// ```sh,ignore
/// $ env RUSTFLAGS='--cfg sqlx_macros_unstable' cargo build
/// ```
///
/// Note that this unfortunately will trigger a fully recompile of your dependency tree, at least
/// for the first time you use it. It also, of course, requires using a nightly compiler.
///
/// You can also set it in `build.rustflags` in `.cargo/config.toml`:
/// ```toml,ignore
/// [build]
/// rustflags = ["--cfg sqlx_macros_unstable"]
/// ```
///
/// And then continue building and running your project normally.
///
/// If you're building on nightly anyways, it would be extremely helpful to help us test
/// this feature and find any bugs in it.
///
/// Subscribe to [the `track_path` tracking issue](https://github.com/rust-lang/rust/issues/73921)
/// for discussion and the future stabilization of this feature.
///
/// For brevity and because it involves the same commitment to unstable features in `proc_macro`,
/// if you're using `--cfg procmacro2_semver_exempt` it will also enable this feature
/// (see [`proc-macro2` docs / Unstable Features](https://docs.rs/proc-macro2/1.0.27/proc_macro2/#unstable-features)).
#[cfg(feature = "migrate")]
#[macro_export]
macro_rules! migrate {
    ($dir:literal) => {{
        $crate::sqlx_macros::migrate!($dir)
    }};

    () => {{
        $crate::sqlx_macros::migrate!("./migrations")
    }};
}
