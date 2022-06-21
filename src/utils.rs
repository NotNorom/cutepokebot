use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateEmbed, ReactionType};
use rs621::post::Post;

/// Create a discord embed from an e6/e9 post
pub fn embed_from_post(post: &Post) -> Result<CreateEmbed, String> {
    Ok(CreateEmbed::default()
        .colour(0x203f6c_u32)
        .title(format!("#{}", post.id))
        .description(&post.description)
        .url(&post.file.url.as_ref().ok_or("No url on post object")?)
        .image(&post.file.url.as_ref().ok_or("No url on post object")?)
        .field("Artist(s)", post.tags.artist.join(", "), false)
        .footer(|footer| {
            let score = format!(
                "up: {}, down: {}, total: {}",
                post.score.up,
                post.score.down,
                post.score.up + post.score.down,
            );
            footer.text(score)
        })
        .to_owned())
}

pub fn post_buttons(current: usize, needed: usize) -> CreateActionRow {
    let mut action_row = CreateActionRow::default();
    action_row.create_button(|downvote_button| {
        downvote_button
            .custom_id("delete-post")
            .emoji(ReactionType::Unicode("❌".to_string()))
            .label(format!("Delete ({}/{})", current, needed))
            .style(ButtonStyle::Primary)
    });

    action_row
}
