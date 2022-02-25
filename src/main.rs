use jolly_bumper::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://127.0.0.1:5432/crates_io")
        .await?;

    let crates = sqlx::query_as::<_, Crate>(
        "SELECT c.id, c.name, c.downloads, c.repository, num as version FROM crates c \
        JOIN versions v on c.id = v.crate_id  \
        WHERE c.downloads > 1000000 \
        ORDER BY downloads DESC;",
    )
    .fetch_all(&pool)
    .await?;

    let count = crates.len();

    let stable_count = crates.iter().filter(|crt| crt.is_stable()).count();

    let unstable_count = count - stable_count;
    let one_percent = count as f32 / 100.0;
    let perc_stable = stable_count as f32 / one_percent;
    let perc_unstable = unstable_count as f32 / one_percent;

    println!(
        "Found {count} crates with more than 1M download on crates.io database. \n\
    Crates with a version lower than 1.0.0 are considered unstable.\n\
     \tStable crates: {stable_count}/{count} - ({perc_stable}%)\n\
     \tUnstable crates: {unstable_count}/{count} - ({perc_unstable}%)"
    );

    Ok(())
}
