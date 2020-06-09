use dashmap::DashMap;
use sqlx;
use sqlx::PgPool;

#[derive(Debug, Default)]
pub struct GuildData {
    pub prefix: String,
    pub commands: DashMap<String, String>
}

pub async fn fetch_guild_data(pool: &PgPool) -> Result<DashMap<i64, GuildData>, Box<dyn std::error::Error>> {
    let guild_info: DashMap<i64, GuildData> = DashMap::new();

    let data = sqlx::query!("SELECT guild_id, prefix FROM guild_info")
        .fetch_optional(pool)
        .await?;

    for i in data {
        let guild_data = add_to_data(pool, i.prefix.unwrap_or_default(), i.guild_id).await?;
        guild_info.insert(i.guild_id, guild_data);
    }

    Ok(guild_info)
}

async fn add_to_data(pool: &PgPool, prefix: String, guild_id: i64) -> Result<GuildData, Box<dyn std::error::Error>> {
    let mut guild_data = GuildData::default();
    let command_map: DashMap<String, String> = DashMap::new();

    let data = sqlx::query!("SELECT * FROM commands WHERE guild_id = $1", guild_id)
        .fetch_optional(pool).await?;

    for i in data {
        println!("Adding: {:?}", i.name);
        command_map.insert(i.name.unwrap(), i.content.unwrap());
    }

    guild_data.commands = command_map;
    guild_data.prefix = prefix;

    Ok(guild_data)
}