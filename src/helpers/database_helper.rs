use dashmap::DashMap;
use serenity::{framework::standard::CommandResult, model::id::GuildId};
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn obtain_db_pool(db_connection: String) -> CommandResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_connection)
        .await?;

    Ok(pool)
}

pub async fn fetch_prefixes(pool: &PgPool) -> CommandResult<DashMap<GuildId, String>> {
    let prefixes: DashMap<GuildId, String> = DashMap::new();

    let cursor = sqlx::query!("SELECT guild_id, prefix FROM guild_info")
        .fetch_all(pool)
        .await?;

    for i in cursor {
        if let Some(prefix) = i.prefix {
            prefixes.insert(GuildId::from(i.guild_id as u64), prefix);
        }
    }

    Ok(prefixes)
}
