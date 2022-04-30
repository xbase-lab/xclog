mod description;
mod error;
mod util;

use async_trait::async_trait;
use std::io;
use util::*;

pub use error::Error;
pub type OutputReader = dyn tokio_stream::Stream<Item = Result<String, io::Error>> + Unpin + Send;

#[async_trait]
pub trait ParsableCompileStep {
    async fn parse(line: String, stream: &mut OutputReader) -> Result<Self, Error>
    where
        Self: Sized + Send;
}
