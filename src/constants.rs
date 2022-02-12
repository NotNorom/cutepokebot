/// in minutes
pub static MINIMUM_TIMEOUT_MINUTES: u64 = 3;
/// in minutes
pub static MAXIMUM_TIMEOUT_MINUTES: u64 = 60 * 12;
/// prefix for all redis requests
pub static REDIS_PREFIX: &str = "e6bot";
/// separator for redis keys
pub static REDIS_PATH_SEPARATOR: &str = "::";
