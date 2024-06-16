use rusql::{Connection, Executor};

use std::time::Instant;

#[derive(rusql::FromRow)]
struct Test {
    id: i32,
}

fn main() -> rusql::Result<()> {
    rusql_rt::block_on(async {
        let mut conn = rusql::SqliteConnection::connect("sqlite://test.db?mode=rwc").await?;
        let delete_sql = "DROP TABLE IF EXISTS test";
        conn.execute(delete_sql).await?;

        let create_sql = "CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY NOT NULL)";
        conn.execute(create_sql).await?;

        let mut tx = conn.begin().await?;
        for entry in 0i32..100000 {
            rusql::query("INSERT INTO test (id) VALUES ($1)")
                .bind(entry)
                .execute(&mut tx)
                .await?;
        }
        tx.commit().await?;

        for _ in 0..10i8 {
            let start = chrono::Utc::now();

            println!(
                "total: {}",
                rusql::query!("SELECT id from test")
                    .fetch_all(&mut conn)
                    .await?
                    .len()
            );

            let elapsed = chrono::Utc::now() - start;
            println!("elapsed {}", elapsed);
        }

        Ok(())
    })
}
