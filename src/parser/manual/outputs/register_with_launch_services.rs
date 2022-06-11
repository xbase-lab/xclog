use crate::parser::util::consume_till_empty_line;
use crate::parser::{Description, Error, OutputStream, ParsableFromStream, Step};
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
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("RegisterWithLaunchServices".into(), "path".into()))?;

        let description = Description::from_line(line)?;

        steps.push(Step::RegisterWithLaunchServices(Self { description, path }));
        steps.extend(consume_till_empty_line(stream).await);

        steps.pipe(Ok)
    }
}

impl Display for RegisterWithLaunchServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} RegisterLaunchServices `{}`",
            self.description,
            self.path.file_name().unwrap().to_string_lossy()
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        RegisterWithLaunchServices,
       r#"RegisterWithLaunchServices $ROOT/build/Release/DemoTarget.app (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    /System/Library/Frameworks/CoreServices.framework/Versions/Current/Frameworks/LaunchServices.framework/Versions/Current/Support/lsregister -f -R -trusted $ROOT/build/Release/DemoTarget.app

"# 
    };

    if let Step::RegisterWithLaunchServices(step) = steps.first().unwrap() {
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
    } else {
        panic!("{steps:#?}")
    }
}
