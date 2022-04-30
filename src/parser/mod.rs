mod description;
mod error;
mod steps;
mod util;

use async_trait::async_trait;
use std::io;

pub use description::Description;
pub use error::Error;
pub use steps::*;
pub use util::*;

pub type OutputStream = dyn tokio_stream::Stream<Item = Result<String, io::Error>> + Unpin + Send;

#[async_trait]
pub trait ParsableFromStream {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error>
    where
        Self: Sized + Send;
}
