use crate::parser::Step;

use super::super::{
    util::consume_till_empty_line, Description, Error, OutputStream, ParsableFromStream,
    ProcessItem,
};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;
use tokio_stream::StreamExt;

/// Code Sign step
#[derive(Debug)]
pub struct CodeSign {
    pub description: Description,
    pub identity: String,
    pub profile: Option<String>,
    pub dir: PathBuf,
    pub sign_key: String,
}

#[async_trait]
impl ParsableFromStream for CodeSign {
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();
        let dir = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CodeSign".into(), "dir".into()))?;

        chunks.next();
        let description = Description::from_line(line)?;

        // Skip exports
        steps.extend(consume_till_empty_line(stream).await);

        let identity = if let Some(ProcessItem::Output(line)) = stream.next().await {
            line.trim()
                .strip_prefix("Signing Identity:")
                .ok_or_else(|| {
                    #[cfg(feature = "with_tracing")]
                    tracing::error!("line: {line}");
                    Error::Failure("Striping identity prefix".into())
                })?
                .trim()
                .replace("\"", "")
        } else {
            return Err(Error::EOF("CodeSign".into(), "identity".into()));
        };

        let profile = if let Some(ProcessItem::Output(line)) = stream.next().await {
            line.trim()
                .strip_prefix("Provisioning Profile:")
                .map(|s| s.trim().replace("\"", ""))
        } else {
            return Err(Error::EOF("CodeSign".into(), "profile".into()));
        };

        // Skip emptry lines
        steps.extend(consume_till_empty_line(stream).await);

        let sign_key = if let Some(ProcessItem::Output(line)) = stream.next().await {
            line.trim()
                .split_whitespace()
                .collect::<Vec<&str>>()
                .get(3)
                .map(ToString::to_string)
                .ok_or_else(|| Error::EOF("CodeSign".into(), "sign_key".into()))?
        } else {
            return Err(Error::EOF("CodeSign".into(), "profile".into()));
        };

        steps.push(Step::CodeSign(Self {
            description,
            identity,
            profile,
            dir,
            sign_key,
        }));

        steps.pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        CodeSign,
r#"CodeSign path/to/build/Debug-iphoneos/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')
    cd /path/to/project
    export CODESIGN_ALLOCATE\=/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/codesign_allocate
    
    Signing Identity:     "Apple Development: email@email.com (XXXXXXXXXX)"
    Provisioning Profile: "iOS Team Provisioning Profile: tami5.DemoProject"
                          (42dd5b24-0395-46bb-bb88-2aed95a7831b)
    
    /usr/bin/codesign --force --sign XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX --entitlements path/to/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/DemoTarget.app.xcent --timestamp\=none --generate-entitlement-der path/to/build/Debug-iphoneos/DemoTarget.app

       "#
    };

    if let Step::CodeSign(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(
            "Apple Development: email@email.com (XXXXXXXXXX)",
            &step.identity
        );
        assert_eq!(
            Some("iOS Team Provisioning Profile: tami5.DemoProject".to_string()),
            step.profile
        );
        assert_eq!("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX", &step.sign_key);
    } else {
        panic!("{steps:#?}")
    }
}

impl Display for CodeSign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Signing `{}`",
            self.description,
            self.dir.file_name().unwrap().to_str().unwrap(),
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn fmt() {
    let data = CodeSign {
        description: Description {
            project: "Project".into(),
            target: "Target".into(),
        },
        identity: "Apple Development: email@email.com (XXXXXXXXXX)".into(),
        profile: Some("iOS Team Provisioning Profile: tami5.DemoProject".into()),
        dir: "/path/to/Target.App".into(),
        sign_key: "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX".into(),
    };

    assert_eq!(
        "[Project.Target] Signing     `Target.App`",
        &format!("{}", data),
    );
}
