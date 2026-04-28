#[cfg(all(feature = "wow-wotlk", not(feature = "replay")))]
pub mod logger;
#[cfg(feature = "wow-wotlk")]
pub mod wotlk;
