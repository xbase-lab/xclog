//! Regex based Parser
#[macro_use]
mod defs;
mod gen;
mod output;

use anyhow::Result;
use process_stream::{ProcessItem, StreamExt};

pub use defs::*;
pub use output::*;

pub(crate) type OutputStream = dyn tokio_stream::Stream<Item = ProcessItem> + Unpin + Send;

/// Process a line with mutable OutputStream
pub async fn parse(line: String, stream: &mut OutputStream) -> Result<Option<Vec<XCOutput>>> {
    if line.contains("ONLY_ACTIVE_ARCH=YES") {
        return Ok(None);
    }

    let matcher = match XCLOG_MATCHER.capture(&line) {
        Some(m) => m,
        None => return Ok(None),
    };

    let mut lines = vec![];
    let line = match matcher.output()? {
        Some(line) => line,
        None => return Ok(None),
    };

    let (is_compile_warning, is_compile_error) =
        (matcher.is_compile_warning(), matcher.is_compile_error());

    if is_compile_warning || is_compile_error {
        let (leading, kind) = if is_compile_error {
            ("[Error]", XCOutputTask::Error)
        } else {
            ("[Warning]", XCOutputTask::Warning)
        };

        let whitespace = XCOutput {
            value: leading.to_string(),
            kind: kind.clone(),
        };

        lines.push(whitespace.clone());
        lines.push(whitespace.clone());

        lines.push(line);

        while let Some(line) = stream.next().await.map(|s| s.to_string()) {
            if line.is_empty() {
                break;
            }
            lines.push(XCOutput {
                value: format!("{leading} {line}"),
                kind: kind.clone(),
            });
        }

        lines.push(whitespace.clone());
        lines.push(whitespace);
    } else {
        lines.push(line);
    }

    Ok(Some(lines))
}

#[tokio::test]
#[tracing_test::traced_test]
#[ignore = "Local tests"]
async fn test_case_2() {
    use crate::logger::XCLogger;
    use process_stream::{ProcessExt, StreamExt};

    let root = "/Users/tami5/repos/swift/yabaimaster";
    let mut logger = XCLogger::new(root, &[
        "clean",
        "build",
        "-configuration",
        "Debug",
        "-target",
        "YabaiMaster",
        "SYMROOT=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "CONFIGURATION_BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug",
        "BUILD_DIR=/Users/tami5/Library/Caches/Xbase/swift_yabaimaster/YabaiMaster_Debug"
    ]).unwrap();

    let mut stream = logger.spawn_and_stream().unwrap();

    while let Some(line) = stream.next().await {
        println!("{}", line)
    }
}
