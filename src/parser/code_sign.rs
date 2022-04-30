use super::{
    description::Description, util::consume_empty_lines, Error, OutputStream, ParsableFromStream,
};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;
use tokio_stream::StreamExt;

/// Code Sign step
#[derive(Debug)]
pub struct CodeSign {
    pub description: Description,
    pub identity: String,
    pub profile: String,
    pub dir: PathBuf,
    pub sign_key: String,
}

#[async_trait]
impl ParsableFromStream for CodeSign {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let dir = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CodeSign".into(), "dir".into()))?;

        chunks.next();
        let description = Description::from_line(line)?;

        // Skip exports
        consume_empty_lines(stream).await;

        let identity = if let Some(Ok(line)) = stream.next().await {
            line.trim()
                .strip_prefix("Signing Identity:")
                .ok_or_else(|| {
                    tracing::error!("line: {line}");
                    Error::Failure("Striping identity prefix".into())
                })?
                .trim()
                .replace("\"", "")
        } else {
            return Err(Error::EOF("CodeSign".into(), "identity".into()));
        };

        let profile = if let Some(Ok(line)) = stream.next().await {
            line.trim()
                .strip_prefix("Provisioning Profile:")
                .ok_or_else(|| Error::Failure("Striping profile prefix".into()))?
                .trim()
                .replace("\"", "")
        } else {
            return Err(Error::EOF("CodeSign".into(), "profile".into()));
        };

        // Skip emptry lines
        consume_empty_lines(stream).await;

        let sign_key = if let Some(Ok(line)) = stream.next().await {
            line.trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .get(3)
                .map(ToString::to_string)
                .ok_or_else(|| Error::EOF("CodeSign".into(), "sign_key".into()))?
        } else {
            return Err(Error::EOF("CodeSign".into(), "profile".into()));
        };

        Self {
            description,
            identity,
            profile,
            dir,
            sign_key,
        }
        .pipe(Ok)
    }
}

#[tokio::test]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let step = to_stream_test! {
        CodeSign,
r#"CodeSign /Users/tami5/repos/swift/wordle/build/Debug-iphoneos/Wordle.app (in target 'DemoTarget' from project 'DemoProject')
    cd /path/to/project
    export CODESIGN_ALLOCATE\=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/codesign_allocate
    
    Signing Identity:     "Apple Development: email@email.com (XXXXXXXXXX)"
    Provisioning Profile: "iOS Team Provisioning Profile: tami5.DemoProject"
                          (42dd5b24-0395-46bb-bb88-2aed95a7831b)
    
    /usr/bin/codesign --force --sign XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX --entitlements /Users/tami5/repos/swift/wordle/build/Wordle.build/Debug-iphoneos/Wordle.build/Wordle.app.xcent --timestamp\=none --generate-entitlement-der /Users/tami5/repos/swift/wordle/build/Debug-iphoneos/Wordle.app

       "#
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        "Apple Development: email@email.com (XXXXXXXXXX)",
        &step.identity
    );
    assert_eq!(
        "iOS Team Provisioning Profile: tami5.DemoProject",
        &step.profile
    );
    assert_eq!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX", &step.sign_key);
}
