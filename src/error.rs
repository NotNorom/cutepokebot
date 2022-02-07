use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("rs621 error")]
    Rs621(#[from] rs621::error::Error),
    #[error("serenity error")]
    Serenity(#[from] poise::serenity_prelude::Error),
    #[error("Command must be run in guild")]
    CommandNotRunInGuild,
    #[error("No tags have been set")]
    NoTagsSet,
}
