#[cfg(feature = "with_regex")]
mod regex;

#[cfg(feature = "with_regex")]
pub use self::regex::*;

#[cfg(feature = "manual")]
mod manual;

#[cfg(feature = "manual")]
pub use self::manual::*;
