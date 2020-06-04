use serenity::model::prelude::*;

use sqlx;
use sqlx::PgPool;
use rand::prelude::*;
use crate::ConnectionPool;

pub fn get_inverted_string(input: &str) -> String {
    
    let mut output = String::with_capacity(input.len());

    for x in input.chars() {
        if x.is_uppercase() {
            output.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
        else if x.is_lowercase() {
            output.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
    }

    output
}

pub fn get_mock_string(input: &str) -> String {

    let mut output = String::with_capacity(input.len());

    for x in input.chars() {
        if random() {
            output.push(x.to_uppercase().collect::<Vec<_>>()[0]);
        }
        else {
            output.push(x.to_lowercase().collect::<Vec<_>>()[0]);
        }
    }

    output
}

pub fn get_spaced_string(input: &str, biggspace: bool) -> String {

    let pass_string: String = input.chars().filter(|c| !c.is_whitespace()).collect();

    pass_string.split("").map(|x|
        if rand::random() {
            if biggspace {
                format!("{}        ", x)
            }

            else {
                format!("{}    ", x)
            }

        } else {
            if biggspace {
                format!("{}     ", x)
            }

            else {
                format!("{}  ", x)
            }
        }).collect::<String>()
}

pub async fn create_channel_row(pool: &PgPool, guild_id: i64, 
    nice_id: impl Into<Option<i64>>, bruh_id: impl Into<Option<i64>>, quote_id: impl Into<Option<i64>>) -> Result<(), Box<dyn std::error::Error>> {

    sqlx::query!("INSERT INTO text_channels VALUES($1, $2, $3, $4)", guild_id, nice_id.into().unwrap_or(0), bruh_id.into().unwrap_or(0), quote_id.into().unwrap_or(0))
        .execute(pool).await?;

    Ok(())
}

pub async fn get_channel(pool: &PgPool, guild_id: GuildId, channel_type: &str) -> Result<i64, Box<dyn std::error::Error>>{

    let mut result: i64 = 0;

    let data = sqlx::query!("SELECT * FROM text_channels WHERE guild_id = $1", guild_id.0 as i64)
        .fetch_optional(pool)
        .await?;
    
    if let Some(data) = data {
        result = match channel_type {
            "nice" => data.nice_id,
            "bruh" => data.bruh_id,
            "quote" => data.quote_id,
            _ => 0 
        };
    }

    Ok(result)
}