use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("rs621 error")]
    Rs621Error(#[from] rs621::error::Error),
    #[error("serenity error")]
    SerenityError(#[from] poise::serenity_prelude::Error),
    #[error("Command must be run in guild")]
    CommandNotRunInGuild,
}
