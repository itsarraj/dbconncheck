use std::process::ExitCode;
use std::time::Instant;

use clap::Parser;
use dbconncheck::scheme::{detect_kind, DbKind};

#[derive(Parser)]
#[command(
    name = "dbconncheck",
    about = "Checks real connectivity, auth, and latency against Postgres, MySQL, or Redis"
)]
struct Cli {
    /// A postgres://, mysql://, or redis:// connection URL.
    url: String,
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    let Some(kind) = detect_kind(&cli.url) else {
        eprintln!("dbconncheck: unrecognized scheme in '{}' — expected postgres://, mysql://, or redis://", cli.url);
        return ExitCode::FAILURE;
    };

    let start = Instant::now();
    let result = match kind {
        DbKind::Postgres => check_postgres(&cli.url).await,
        DbKind::MySql => check_mysql(&cli.url).await,
        DbKind::Redis => check_redis(&cli.url).await,
    };
    let elapsed = start.elapsed();

    match result {
        Ok(()) => {
            println!("OK: connected and authenticated in {elapsed:?}");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("FAILED after {elapsed:?}: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn check_postgres(url: &str) -> anyhow::Result<()> {
    use sqlx::Connection;
    let mut conn = sqlx::PgConnection::connect(url).await?;
    sqlx::query("SELECT 1").execute(&mut conn).await?;
    Ok(())
}

async fn check_mysql(url: &str) -> anyhow::Result<()> {
    use sqlx::Connection;
    let mut conn = sqlx::MySqlConnection::connect(url).await?;
    sqlx::query("SELECT 1").execute(&mut conn).await?;
    Ok(())
}

async fn check_redis(url: &str) -> anyhow::Result<()> {
    let client = redis::Client::open(url)?;
    let mut con = client.get_multiplexed_async_connection().await?;
    let _: String = redis::cmd("PING").query_async(&mut con).await?;
    Ok(())
}
