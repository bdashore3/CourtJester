use dashmap::DashMap;
use sqlx;
use sqlx::PgPool;

pub async fn fetch_prefixes(pool: &PgPool) -> Result<DashMap<i64, String>, Box<dyn std::error::Error>> {
    let prefixes = DashMap::new();

    let cursor = sqlx::query!("SELECT guild_id, prefix FROM guild_info")
        .fetch_optional(pool)
        .await?;

    for i in cursor {
        prefixes.insert(i.guild_id, i.prefix.unwrap());
    }

    Ok(prefixes)
}
