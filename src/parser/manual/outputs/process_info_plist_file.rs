use crate::parser::{
    consume_till_empty_line, Description, Error, OutputStream, ParsableFromStream, Step,
};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;

/// Info plist process step
#[derive(Debug)]
pub struct ProcessInfoPlistFile {
    pub description: Description,
    pub path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for ProcessInfoPlistFile {
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();
        chunks.next();
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("ProcessInfoPlistFile".into(), "path".into()))?;

        let description = Description::from_line(line)?;
        steps.push(Step::ProcessInfoPlistFile(Self { description, path }));
        steps.extend(consume_till_empty_line(stream).await);

        steps.pipe(Ok)
    }
}

impl Display for ProcessInfoPlistFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Processing `{}`",
            self.description,
            self.path
                .strip_prefix(self.path.ancestors().nth(2).unwrap())
                .unwrap()
                .display()
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        ProcessInfoPlistFile,
       r#"ProcessInfoPlistFile $ROOT/build/Debug-iphoneos/DemoTarget.app/Info.plist $ROOT/Resources/Info.plist (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    builtin-infoPlistUtility $ROOT/Resources/Info.plist -producttype com.apple.product-type.application -genpkginfo $ROOT/build/Debug-iphoneos/DemoTarget.app/PkgInfo -expandbuildsettings -format binary -platform iphoneos -additionalcontentfile $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/LaunchScreen-SBPartialInfo.plist -additionalcontentfile $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/assetcatalog_generated_info.plist -requiredArchitecture arm64 -o $ROOT/build/Debug-iphoneos/DemoTarget.app/Info.plist

"# 
    };
    if let Step::ProcessInfoPlistFile(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(PathBuf::from("$ROOT/Resources/Info.plist"), step.path);
        assert_eq!(
            step.to_string(),
            "[DemoProject.DemoTarget] Processing `Resources/Info.plist`"
        );
    } else {
        panic!("No script execution {steps:#?}")
    }
}
