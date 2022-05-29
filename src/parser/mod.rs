#[cfg(feature = "regex")]
mod regex;

#[cfg(feature = "regex")]
pub use self::regex::*;

#[cfg(feature = "manual")]
mod manual;

#[cfg(feature = "manual")]
pub use self::manual::*;
