use serenity::model::prelude::*;

use sqlx;
use sqlx::PgPool;
use rand::prelude::*;
use crate::ConnectionPool;

// Switches the case of each character in the word and returns the new word
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

/*
 * Makes a spongebob cased string
 * Takes a random value and either makes the letter uppercase or lowercase
 * There is a chance it will output an uppercased or original string due to probability
 */
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

/*
 * Adds x amount of spaces between each character of the string. Whitespace is trimmed at collection
 * If biggspace is true, add a larger space between each character
 */
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

// Create a row in the db based on the given channel id. The others are optional and don't have to be provided
pub async fn create_channel_row(pool: &PgPool, guild_id: i64, 
    nice_id: impl Into<Option<i64>>, bruh_id: impl Into<Option<i64>>, 
    quote_id: impl Into<Option<i64>>) -> Result<(), Box<dyn std::error::Error>> {

    sqlx::query!("INSERT INTO text_channels VALUES($1, $2, $3, $4)", guild_id, nice_id.into().unwrap_or(0), bruh_id.into().unwrap_or(0), quote_id.into().unwrap_or(0))
        .execute(pool).await?;

    Ok(())
}

// Get the channel Id to send a message to whether it's nice, a quote, or bruh
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