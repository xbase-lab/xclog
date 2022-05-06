use crate::parser::{Error, OutputStream, ParsableFromStream};
use crate::runner::ProcessUpdate;
use async_trait::async_trait;
use std::fmt::Display;
use tap::Pipe;
use tokio_stream::StreamExt;

#[derive(Debug, PartialEq, Eq)]
pub struct Package {
    pub url: String,
    pub name: String,
    pub version: String,
}

/// Linking of a library
#[derive(Debug)]
pub struct ResolvedSourcePackages {
    pub packages: Vec<Package>,
}

#[async_trait]
impl ParsableFromStream for ResolvedSourcePackages {
    async fn parse_from_stream(_: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut packages = Vec::new();

        while let Some(ProcessUpdate::Stdout(line)) = stream.next().await {
            let line = line.trim();
            if line.is_empty() {
                break;
            }

            let mut chunks = line.split_whitespace();

            let (mut name, mut version, mut url) = (None, None, None);
            while let Some(chunk) = chunks.next() {
                if chunk.ends_with(":") {
                    name = chunk.replace(":", "").into();
                    url = chunks.next().map(ToString::to_string);
                }
                if chunk.eq("@") {
                    version = chunks.next().map(ToString::to_string);
                }
            }
            let name =
                name.ok_or_else(|| Error::EOF("ResolvedSourcePackages".into(), "name".into()))?;
            let url =
                url.ok_or_else(|| Error::EOF("ResolvedSourcePackages".into(), "url".into()))?;
            let version = version
                .ok_or_else(|| Error::EOF("ResolvedSourcePackages".into(), "version".into()))?;

            packages.push(Package { url, name, version });
        }

        Self { packages }.pipe(Ok)
    }
}

impl Display for ResolvedSourcePackages {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "[Packages] ------------------------------------------------------------"
        )?;
        {
            let mut sorted = self.packages.iter().collect::<Vec<_>>();
            sorted.sort_by_key(|a| &a.name);
            sorted
        }
        .iter()
        .try_for_each(|package| {
            writeln!(
                f,
                "[Packages]  `{}`  ({}) => {}",
                package.name, package.version, package.url
            )
        })?;
        write!(
            f,
            "[Packages] -------------------------------------------------------------"
        )?;
        Ok(())
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        ResolvedSourcePackages,
       r#"Resolved source packages:
  swift-argument-parser: https://github.com/apple/swift-argument-parser @ main
  Socket: https://github.com/Kitura/BlueSocket @ 2.0.2
  SwiftyBeaver: https://github.com/SwiftyBeaver/SwiftyBeaver.git @ 1.9.5

"# 
    };
    assert_eq!(
        step.packages,
        vec![
            Package {
                name: "swift-argument-parser".to_string(),
                url: "https://github.com/apple/swift-argument-parser".to_string(),
                version: "main".to_string()
            },
            Package {
                name: "Socket".to_string(),
                url: "https://github.com/Kitura/BlueSocket".to_string(),
                version: "2.0.2".to_string()
            },
            Package {
                name: "SwiftyBeaver".to_string(),
                url: "https://github.com/SwiftyBeaver/SwiftyBeaver.git".to_string(),
                version: "1.9.5".to_string()
            },
        ]
    );
}
