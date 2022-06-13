use process_stream::ProcessItem;
pub type OutputStream = dyn tokio_stream::Stream<Item = ProcessItem> + Unpin + Send;

#[cfg(feature = "with_regex")]
mod regex;

#[cfg(feature = "with_regex")]
pub use self::regex::*;

#[cfg(feature = "manual")]
mod manual;

#[cfg(feature = "manual")]
pub use self::manual::*;
