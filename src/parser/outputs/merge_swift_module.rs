use crate::parser::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;

/// Resource file was copied
#[derive(Debug)]
pub struct MergeSwiftModule {
    pub arch: String,
    pub description: Description,
    pub output_path: PathBuf,
}

#[async_trait]
impl ParsableFromStream for MergeSwiftModule {
    async fn parse_from_stream(line: String, _stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        chunks.next();

        Self {
            arch: chunks
                .next()
                .map(ToString::to_string)
                .ok_or_else(|| Error::EOF("MergeSwiftModule".into(), "arch".into()))?,
            output_path: chunks
                .next()
                .map(PathBuf::from)
                .ok_or_else(|| Error::EOF("MergeSwiftModule".into(), "output_path".into()))?,
            description: Description::from_line(line)?,
        }
        .pipe(Ok)
    }
}

impl Display for MergeSwiftModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} Merging    {}",
            self.description,
            self.output_path.file_name().unwrap().to_string_lossy()
        )
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        MergeSwiftModule,
       r#"MergeSwiftModule normal x86_64 /path/to/build/Objects-normal/x86_64/helloworld.swiftmodule (in target 'DemoTarget' from project 'DemoProject')
        cd $ROOT
        $TOOLCHAIN_BIN/swift -frontend -merge-modules -emit-module /path/to/build/Objects-normal/x86_64/ViewController~partial.swiftmodule /path/to/build/Objects-normal/x86_64/AppDelegate~partial.swiftmodule -parse-as-library -sil-merge-partial-modules -disable-diagnostic-passes -disable-sil-perf-optzns -target x86_64-apple-ios10.2 -enable-objc-interop -sdk path/to/platforms/iPhoneSimulator.platform/Developer/SDKs/iPhoneSimulator11.3.sdk -I path/to/Products/Debug-iphonesimulator -F path/to/Products/Debug-iphonesimulator -enable-testing -g -module-cache-path /path/to/DerivedData/ModuleCache.noindex -swift-version 4 -enforce-exclusivity=checked -Onone -D DEBUG -serialize-debugging-options -Xcc -I/path/to/build/swift-overrides.hmap -Xcc -iquote -Xcc /path/to/build/helloworld-generated-files.hmap -Xcc -I/path/to/build/helloworld-own-target-headers.hmap -Xcc -I/path/to/build/helloworld-all-target-headers.hmap -Xcc -iquote -Xcc /path/to/build/helloworld-project-headers.hmap -Xcc -Ipath/to/Products/Debug-iphonesimulator/include -Xcc -I/path/to/build/DerivedSources/x86_64 -Xcc -I/path/to/build/DerivedSources -Xcc -DDEBUG=1 -Xcc -working-directory$ROOT -emit-module-doc-path /path/to/build/Objects-normal/x86_64/helloworld.swiftdoc -module-name helloworld -emit-objc-header-path /path/to/build/Objects-normal/x86_64/helloworld-Swift.h -o /path/to/build/Objects-normal/x86_64/helloworld.swiftmodule

"# 
    };

    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!(
        PathBuf::from("/path/to/build/Objects-normal/x86_64/helloworld.swiftmodule"),
        step.output_path
    );
    assert_eq!(
        "[DemoProject.DemoTarget] Merging    `helloworld.swiftmodule`",
        step.to_string()
    );
}
