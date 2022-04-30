use super::{Error, OutputReader};
use tokio_stream::StreamExt;

/// Ignore stream content til the line is empty
pub async fn consume_empty_lines(stream: &mut OutputReader) {
    while let Ok(Some(line)) = stream.try_next().await {
        if line.trim().is_empty() {
            break;
        }
    }
}
