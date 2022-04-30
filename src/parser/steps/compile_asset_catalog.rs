use super::super::{Description, Error, OutputStream, ParsableFromStream};
use async_trait::async_trait;
use std::path::PathBuf;
use tap::Pipe;
use tokio_stream::StreamExt;

/// Asset's catalog compilation
#[derive(Debug)]
pub struct CompileAssetCatalog {
    pub path: PathBuf,
    pub description: Description,
    pub results: Vec<PathBuf>,
    pub notices: Vec<String>,
}

#[async_trait]
impl ParsableFromStream for CompileAssetCatalog {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        chunks.next(); // Skip root
        let path = chunks
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Error::EOF("CompileAssetCatalog".into(), "path".into()))?;

        chunks.next();
        let description = Description::from_line(line)?;
        let (mut notices, mut results) = (vec![], vec![]);

        while let Some(Ok(line)) = stream.next().await {
            let mut line = line.trim().to_string();

            if line.is_empty() {
                break;
            }

            if line.contains("com.apple.actool.document.notices") {
                while let Some(Ok(maybe_notice)) = stream.next().await {
                    if maybe_notice.starts_with("/*") {
                        line = maybe_notice;
                        break;
                    }
                    maybe_notice
                        .trim()
                        .split(":")
                        .skip(3)
                        .collect::<String>()
                        .trim()
                        .to_string()
                        .pipe(|notice| notices.push(notice));
                }
            }

            if line.contains("com.apple.actool.compilation-results") {
                while let Some(Ok(maybe_path)) = stream.next().await {
                    let maybe_path = maybe_path.trim();
                    if !maybe_path.starts_with("/") {
                        break;
                    }
                    results.push(PathBuf::from(maybe_path))
                }
            }
        }

        Self {
            description,
            path,
            results,
            notices,
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
        CompileAssetCatalog,
r#"CompileAssetCatalog /path/to/build/Debug-iphoneos/TargetName.app /path/to/resources/Assets.xcassets (in target 'TargetName' from project 'ProjectName')
    cd /path/to/project
    /Applications/Xcode.app/Contents/Developer/usr/bin/actool --output-format human-readable-text --notices --warnings --export-dependency-info /path/to/build/TargetName.build/Debug-iphoneos/TargetName.build/assetcatalog_dependencies --output-partial-info-plist /path/to/build/TargetName.build/Debug-iphoneos/TargetName.build/assetcatalog_generated_info.plist --app-icon AppIcon --compress-pngs --enable-on-demand-resources YES --development-region en --target-device iphone --target-device ipad --minimum-deployment-target 15.0 --platform iphoneos --compile /path/to/build/Debug-iphoneos/TargetName.app /path/to/resources/Assets.xcassets
2022-04-28 20:00:56.701 ibtoold[26330:10896383] DEBUG: Added to environment: {
    TMPDIR = "/var/folders/lm/jgnf6c7941qbrz4r6j5qscx00000gn/T/B69BD433-9BEC-4F5B-BF62-48A4B0DE4C88";
}
/* com.apple.actool.document.notices */
/path/to/resources/Assets.xcassets:./AppIcon.appiconset/[][ipad][76x76][][][1x][][][]: notice: 76x76@1x app icons only apply to iPad apps targeting releases of iOS prior to 10.0.
/* com.apple.actool.compilation-results */
/path/to/build/Debug-iphoneos/TargetName.app/AppIcon60x60@2x.png
/path/to/build/Debug-iphoneos/TargetName.app/AppIcon76x76@2x~ipad.png
/path/to/build/Debug-iphoneos/TargetName.app/Assets.car
/path/to/build/TargetName.build/Debug-iphoneos/TargetName.build/assetcatalog_generated_info.plist

"#
    };
    assert_eq!("TargetName", &step.description.target);
    assert_eq!("ProjectName", &step.description.project);
    assert_eq!(
        PathBuf::from("/path/to/resources/Assets.xcassets"),
        step.path
    );
    assert_eq!(
        vec![PathBuf::from("/path/to/build/Debug-iphoneos/TargetName.app/AppIcon60x60@2x.png"),
        PathBuf::from("/path/to/build/Debug-iphoneos/TargetName.app/AppIcon76x76@2x~ipad.png"),
        PathBuf::from("/path/to/build/Debug-iphoneos/TargetName.app/Assets.car"),
        PathBuf::from("/path/to/build/TargetName.build/Debug-iphoneos/TargetName.build/assetcatalog_generated_info.plist")
        ],
        step.results
    );
    assert_eq!(
        vec![
            "76x76@1x app icons only apply to iPad apps targeting releases of iOS prior to 10.0."
                .to_string()
        ],
        step.notices
    );
}
