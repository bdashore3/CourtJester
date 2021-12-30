use serenity::builder::CreateEmbed;

use crate::{helpers::command_utils::truncate, structures::JapanResult};

pub fn get_result_embed(result_string: &str) -> CreateEmbed {
    let mut eb = CreateEmbed::default();

    eb.description(format!("Here are your results \n\n```{}```", result_string));
    eb.footer(|f| {
        f.text("Pick a number from the list. \nType abort if you want to cancel");
        f
    });

    eb
}

pub fn get_anime_embed(japan_result: &JapanResult) -> CreateEmbed {
    let synopsis = truncate(&japan_result.synopsis, 40);
    let mut eb = CreateEmbed::default();

    eb.title(&japan_result.title);
    eb.thumbnail(&japan_result.main_picture.medium);
    eb.description(synopsis);

    // Is this an anime or manga?
    if let Some(episodes) = japan_result.num_episodes {
        eb.field("Episodes", episodes, true);

        if japan_result.status == "finished_airing" {
            eb.field("Status", "Complete", true);
        } else {
            eb.field("Status", "Airing", true);
        }

        eb.url(format!("https://myanimelist.net/anime/{}", japan_result.id));

        eb.color(0xff004c);
    } else if let (Some(chapters), Some(volumes)) =
        (japan_result.num_chapters, japan_result.num_volumes)
    {
        if chapters == 0 {
            eb.field("Chapters", "Unknown", true);
        } else {
            eb.field("Chapters", chapters, true);
        }

        if volumes == 0 {
            eb.field("Chapters", "Unknown", true);
        } else {
            eb.field("Chapters", volumes, true);
        }

        if japan_result.status == "finished" {
            eb.field("Status", "Complete", true);
        } else {
            eb.field("Status", "Publishing", true);
        }

        eb.url(format!("https://myanimelist.net/manga/{}", japan_result.id));

        eb.color(0x00e1ff);
    }

    if let Some(score) = japan_result.mean {
        eb.field("MAL Score", score, true);
    } else {
        eb.field("MAL Score", "Unknown", true);
    };

    eb.footer(|f| {
        f.text("Data provided by the MyAnimeList API");
        f
    });

    eb
}
