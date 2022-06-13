#[macro_use]
mod define;
mod generate;
mod output;

use anyhow::Result;
use process_stream::{ProcessItem, StreamExt};

pub use generate::*;
pub use output::*;

pub(crate) type OutputStream = dyn tokio_stream::Stream<Item = ProcessItem> + Unpin + Send;

pub(crate) async fn parse(line: String, stream: &mut OutputStream) -> Result<Option<Vec<String>>> {
    if line.contains("ONLY_ACTIVE_ARCH=YES") {
        return Ok(None);
    }

    let matcher = match MATCHER.capture(&line) {
        Some(m) => m,
        None => return Ok(None),
    };

    let mut lines = vec![];
    let line = match matcher.output()?.value {
        Some(line) => line,
        None => return Ok(None),
    };

    let (is_compile_warning, is_compile_error) =
        (matcher.is_compile_warning(), matcher.is_compile_error());

    if is_compile_warning || is_compile_error {
        let leading = if is_compile_error {
            "[Error]"
        } else {
            "[Warning]"
        };
        lines.push(leading.to_string());
        lines.push(leading.to_string());
        lines.push(line);
        while let Some(line) = stream.next().await.map(|s| s.to_string()) {
            if line.is_empty() {
                break;
            }
            lines.push(format!("{leading} {line}"));
        }
        lines.push(leading.to_string());
        lines.push(leading.to_string());
    } else {
        lines.push(line);
    }

    Ok(Some(lines))
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = "Local tests"]
async fn test_case_2() {
    use crate::get_log_stream;
    use process_stream::StreamExt;

    let root = "/Users/tami5/repos/swift/yabaimaster";
    let mut stream = get_log_stream(root, &[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
    ]).await.unwrap();

    while let Some(line) = StreamExt::next(&mut stream).await {
        println!("{}", line)
    }
}
