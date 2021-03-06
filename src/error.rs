use thiserror::Error;


/// The error type for this crate.
/// 
/// Includes error for dependencies as well.
#[derive(Error, Debug)]
pub enum Error {
    #[error("rs621 error")]
    Rs621(#[from] rs621::error::Error),
    #[error("serenity error")]
    Serenity(#[from] poise::serenity_prelude::Error),
    #[error("redis error")]
    Redis(#[from] fred::error::RedisError),
    #[error("Command must be run in guild")]
    CommandNotRunInGuild,
    #[error("No tags have been set")]
    NoTagsSet,
    #[error("uhhh")]
    Uhhh(String),
    #[error("Min timeout is too low")]
    MinTimeoutTooLow,
    #[error("Max Timeout is too high")]
    MaxTimeoutTooHigh,
}


pub enum ArgumentError {
    
}