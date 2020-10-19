use serenity::{
    prelude::*,
    framework::standard::CommandResult,
};
use crate::ConnectionPool;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::time::delay_for;


pub async fn starboard_removal_loop(ctx: Context) -> CommandResult {
    let pool = ctx.data.read().await
        .get::<ConnectionPool>().cloned().unwrap();

    loop {    
        let delete_data = sqlx::query!("SELECT guild_id, reaction_message_id, sent_message_id, delete_time FROM starboard")
            .fetch_all(&pool).await?;
        
        for i in delete_data {
            println!("Checking delete status on starboard message {}", i.guild_id);

            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards?")
                .as_secs() as i64;

            if i.delete_time <= current_time {
                println!("Deleting guild {} from the database \n", i.guild_id);
                sqlx::query!("DELETE FROM starboard WHERE guild_id = $1 AND reaction_message_id = $2 AND sent_message_id = $3",
                        i.guild_id, i.reaction_message_id, i.sent_message_id)
                    .execute(&pool).await?;
            } else {
                println!("Entry's time isn't greater than a week! Not deleting guild {}! \n", i.guild_id);
            }
        }
    
        delay_for(Duration::from_secs(345600)).await;
    }
}
