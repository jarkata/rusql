use rusql_core as rusql;

impl_database_ext! {
    rusql::mssql::Mssql {
        bool,
        i8,
        i16,
        i32,
        i64,
        f32,
        f64,
        String,
    },
    ParamChecking::Weak,
    feature-types: _info => None,
    row = rusql::mssql::MssqlRow,
    name = "MSSQL"
}
