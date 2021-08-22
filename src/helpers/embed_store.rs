use serenity::builder::CreateEmbed;

use crate::structures::{AnimeResult, MangaResult};

pub fn get_result_embed(result_string: &str) -> CreateEmbed {
    let mut eb = CreateEmbed::default();

    eb.description(format!("Here are your results \n\n```{}```", result_string));
    eb.footer(|f| {
        f.text("Pick a number from the list. \nType abort if you want to cancel");
        f
    });

    eb
}

pub fn get_anime_embed(anime: &AnimeResult) -> CreateEmbed {
    let mut eb = CreateEmbed::default();

    eb.color(0xff004c);
    eb.title(&anime.title);
    eb.url(&anime.url);
    eb.thumbnail(&anime.image_url);
    eb.description(&anime.synopsis);
    eb.field("Episodes", anime.episodes, true);

    if anime.airing {
        eb.field("Status", "Airing", true);
    } else {
        eb.field("Status", "Complete", true);
    }

    eb.field("MAL Score", &anime.score, true);

    eb.footer(|f| {
        f.text("Data provided by the Jikan API");
        f
    });

    eb
}

pub fn get_manga_embed(manga: &MangaResult) -> CreateEmbed {
    let mut eb = CreateEmbed::default();

    eb.color(0x00e1ff);
    eb.title(&manga.title);
    eb.url(&manga.url);
    eb.thumbnail(&manga.image_url);
    eb.description(&manga.synopsis);

    if manga.chapters == 0 {
        eb.field("Chapters", "Unknown", true);
    } else {
        eb.field("Chapters", manga.chapters, true);
    }

    if manga.volumes == 0 {
        eb.field("Volumes", "Unknown", true);
    } else {
        eb.field("Volumes", manga.volumes, true);
    }

    if manga.publishing {
        eb.field("Status", "Publishing", true);
    } else {
        eb.field("Status", "Complete", true);
    }

    eb.field("MAL Score", &manga.score, true);

    eb.footer(|f| {
        f.text("Data provided by the Jikan API");
        f
    });

    eb
}
