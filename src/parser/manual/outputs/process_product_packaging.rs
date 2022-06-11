use crate::parser::{
    consume_till_empty_line, Description, Error, OutputStream, ParsableFromStream, Step,
};
use async_trait::async_trait;
use process_stream::ProcessItem;
use std::{collections::HashMap, fmt::Display};
use tap::Pipe;
use tokio_stream::StreamExt;

/// Packaging step
#[derive(Debug)]
pub struct ProcessProductPackaging {
    pub description: Description,
    pub entitlements: HashMap<String, String>,
}

#[async_trait]
impl ParsableFromStream for ProcessProductPackaging {
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut entitlements = HashMap::default();
        let description = Description::from_line(line)?;

        while let Some(s) = stream.next().await {
            if let ProcessItem::Output(line) = s {
                if line.contains("Entitlements") {
                    while let Some(s) = stream.next().await {
                        if let ProcessItem::Output(line) = s {
                            let line = line.trim();
                            if line.starts_with("builtin-productPackagingUtility") {
                                break;
                            }
                            if line.starts_with('"') {
                                let kv = line.split("=").collect::<Vec<&str>>();
                                let key = kv.get(0).map(|s| s.trim().replace('"', "")).ok_or_else(
                                    || {
                                        Error::EOF(
                                            "ProcessProductPackaging".into(),
                                            "entitlements key".into(),
                                        )
                                    },
                                )?;
                                let value = kv
                                    .get(1)
                                    .map(|s| s.trim().replace('"', "").replace(";", ""))
                                    .ok_or_else(|| {
                                        Error::EOF(
                                            "ProcessProductPackaging".into(),
                                            "entitlements value".into(),
                                        )
                                    })?;
                                entitlements.insert(key, value);
                            }
                            if line.starts_with("}") {
                                break;
                            }
                        } else if let ProcessItem::Error(line) = s {
                            steps.push(Step::Error(line));
                        }
                    }
                }
            } else if let ProcessItem::Error(line) = s {
                steps.push(Step::Error(line));
            } else if s.trim().starts_with("builtin-productPackagingUtility") {
                break;
            }
        }

        steps.push(Step::ProcessProductPackaging(Self {
            description,
            entitlements,
        }));

        steps.extend(consume_till_empty_line(stream).await);

        steps.pipe(Ok)
    }
}
impl Display for ProcessProductPackaging {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = self.description.to_string();
        {
            let mut sorted = self.entitlements.iter().collect::<Vec<_>>();
            sorted.sort_by_key(|a| a.0);
            sorted
        }
        .iter()
        .try_for_each(|(key, value)| writeln!(f, "{} [{}]: {}", description, key, value))?;

        Ok(())
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        ProcessProductPackaging,
       r#"ProcessProductPackaging "" $ROOT/build/DemoTarget.build/Release/DemoTarget.build/DemoTarget.app.xcent (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    
    Entitlements:
    
    {
    "com.apple.security.get-task-allow" = 1;
}
    
    builtin-productPackagingUtility -entitlements -format xml -o $ROOT/build/DemoTarget.build/Release/DemoTarget.build/DemoTarget.app.xcent

"# 
    };

    if let Step::ProcessProductPackaging(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(
            HashMap::from([(
                "com.apple.security.get-task-allow".to_string(),
                "1".to_string()
            )]),
            step.entitlements
        );
        assert_eq!("[DemoProject.DemoTarget] Packaging   ----------------------------------------------------\n[DemoProject.DemoTarget] Entitlement `com.apple.security.get-task-allow` = 1\n[DemoProject.DemoTarget] Packaging   ----------------------------------------------------", step.to_string());
    } else {
        panic!("{steps:#?}")
    }

    let steps = to_stream_test! {
        ProcessProductPackaging,
r#"ProcessProductPackaging "" $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/DemoTarget.app.xcent (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT
    
    Entitlements:
    
    {
    "application-identifier" = "7N5BMV2F5G.tami5.DemoTarget";
    "com.apple.developer.team-identifier" = 7N5BMV2F5G;
    "get-task-allow" = 1;
}
    
    builtin-productPackagingUtility -entitlements -format xml -o $ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/DemoTarget.app.xcent

"#
    };
    if let Step::ProcessProductPackaging(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!(
            HashMap::from([
                (
                    "application-identifier".to_string(),
                    "7N5BMV2F5G.tami5.DemoTarget".to_string()
                ),
                (
                    "com.apple.developer.team-identifier".to_string(),
                    "7N5BMV2F5G".to_string()
                ),
                ("get-task-allow".to_string(), "1".to_string())
            ]),
            step.entitlements
        );
        assert_eq!("[DemoProject.DemoTarget] Packaging   ----------------------------------------------------n[DemoProject.DemoTarget] Entitlement `application-identifier` = 7N5BMV2F5G.tami5.DemoTarget\n[DemoProject.DemoTarget] Entitlement `com.apple.developer.team-identifier` = 7N5BMV2F5G\n[DemoProject.DemoTarget] Entitlement `get-task-allow` = 1\n[DemoProject.DemoTarget] Packaging   ----------------------------------------------------", step.to_string())
    } else {
        panic!("{steps:#?}")
    }
}
