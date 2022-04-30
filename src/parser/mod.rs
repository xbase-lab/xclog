mod code_sign;
mod compile_asset_catalog;
mod compile_c;
mod compile_swift;
mod compile_swift_sources;
mod description;
mod error;
mod invocation;
mod step;
mod util;

use async_trait::async_trait;
use std::io;

pub use error::Error;
pub use util::*;

pub type OutputStream = dyn tokio_stream::Stream<Item = Result<String, io::Error>> + Unpin + Send;

#[async_trait]
pub trait ParsableFromStream {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error>
    where
        Self: Sized + Send;
}
