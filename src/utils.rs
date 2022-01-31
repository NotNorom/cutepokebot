use poise::serenity_prelude::CreateEmbed;
use rs621::post::Post;

pub fn embed_from_post(post: &Post) -> Result<CreateEmbed, String> {
    Ok(CreateEmbed::default()
        .colour(0x203f6c_u32)
        .title(format!("#{}", post.id))
        .description(&post.description)
        .url(
            &post
                .file
                .url
                .as_ref()
                .ok_or_else(|| "No url on post object")?,
        )
        .image(
            &post
                .file
                .url
                .as_ref()
                .ok_or_else(|| "No url on post object")?,
        )
        .field(
            "Artist(s)",
            format!("{}", post.tags.artist.join(", ")),
            false,
        )
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
