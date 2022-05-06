use crate::parser::util::consume_till_empty_line;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::fmt::Display;
use std::path::PathBuf;
use tap::Pipe;

/// Linking of a library
#[derive(Debug)]
pub struct RegisterWithLaunchServices {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for RegisterWithLaunchServices {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("RegisterWithLaunchServices".into(), "path".into()))?;

        let description = Description::from_line(line)?;

        consume_till_empty_line(stream).await;

        Self { description, path }.pipe(Ok)
    }
}

impl Display for RegisterWithLaunchServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} RegisterLaunchServices     {}",
            self.description,
            self.path.file_name().unwrap().to_string_lossy()
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        RegisterWithLaunchServices,
       r#"RegisterWithLaunchServices $ROOT/build/Release/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    /System/Library/Frameworks/CoreServices.framework/Versions/Current/Frameworks/LaunchServices.framework/Versions/Current/Support/lsregister -f -R -trusted $ROOT/build/Release/DemoTarget.app

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("$ROOT/build/Debug-iphoneos/DemoTarget.app/DemoTarget"),
        step.path
    );

    assert_eq!(
        "[DemoProject.DemoTarget] RegisterLaunchServices   `DemoTarget`",
        step.to_string()
    )
}
