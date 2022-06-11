use crate::parser::{Description, Error, OutputStream, ParsableFromStream, Step};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;

/// Resource file was copied
#[derive(Debug)]
pub struct CopyResource {
    pub description: Description,
    pub path: PathBuf,
    pub output_path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for CopyResource {
    async fn parse_from_stream(
        line: String,
        _stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut chunks = line.split_whitespace();
        vec![Step::CopyResource(Self {
            output_path: chunks
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| Error::EOF("GenerateDSYMFile".into(), "output_path".into()))?,
            path: chunks
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| Error::EOF("GenerateDSYMFile".into(), "path".into()))?,
            description: Description::from_line(line)?,
        })]
        .pipe(Ok)
    }
}

impl Display for CopyResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Copying `{}`",
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
        CopyResource,
       r#"CpResource $ROOT/build/Debug-iphoneos/DemoTarget.app/EnWords.txt $ROOT/Resources/EnWords.txt (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    builtin-copy -exclude .DS_Store -exclude CVS -exclude .svn -exclude .git -exclude .hg -resolve-src-symlinks $ROOT/Resources/EnWords.txt $ROOT/build/Debug-iphoneos/DemoTarget.app

"# 
    };
    if let Step::CopyResource(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(PathBuf::from("$ROOT/Resources/EnWords.txt"), step.path);
        assert_eq!(
            PathBuf::from("$ROOT/build/Debug-iphoneos/DemoTarget.app/EnWords.txt"),
            step.output_path
        );
        assert_eq!(
            "[DemoProject.DemoTarget] Copying   `Resources/EnWords.txt`",
            step.to_string()
        )
    } else {
        panic!("No script execution {steps:#?}")
    }
}
