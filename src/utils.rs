use std::fmt::Display;

use poise::{
    serenity_prelude::{ButtonStyle, CreateActionRow, CreateEmbed, ReactionType},
    SlashChoiceParameter,
};
use rs621::post::Post;
use tracing::instrument;

#[instrument]
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

pub fn post_buttons() -> CreateActionRow {
    let mut action_row = CreateActionRow::default();
    action_row.create_button(|downvote_button| {
        downvote_button
            .custom_id("delte-post")
            .emoji(ReactionType::Unicode("❌".to_string()))
            .label("delete")
            .style(ButtonStyle::Primary)
    });

    action_row
}

/// NSFW mode. Default is SFW
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, SlashChoiceParameter)]
pub enum NsfwMode {
    #[name = "sfw"]
    SFW,
    #[name = "nsfw"]
    NSFW,
}

impl Default for NsfwMode {
    fn default() -> Self {
        Self::SFW
    }
}

impl Display for NsfwMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NsfwMode::SFW => write!(f, "sfw"),
            NsfwMode::NSFW => write!(f, "nsfw"),
        }
    }
}
