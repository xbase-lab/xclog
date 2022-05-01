use crate::parser::{
    consume_till_empty_line, Description, Error, OutputStream, ParsableFromStream,
};
use async_trait::async_trait;
use tap::Pipe;

/// Build phase shell script execution
#[derive(Debug)]
pub struct ScriptExecution {
    pub name: String,
    pub description: Description,
}

#[async_trait]
impl ParsableFromStream for ScriptExecution {
    async fn parse_from_stream(line: String, stream: &mut OutputStream) -> Result<Self, Error> {
        let mut chunks = line.split_whitespace();
        let mut name = vec![];

        // NOTE: relays on `/`
        while let Some(chunk) = chunks.next() {
            if chunk.starts_with("/") {
                break;
            }
            name.push(chunk.replace("\\", ""))
        }

        let description = Description::from_line(line)?;

        consume_till_empty_line(stream).await;

        Self {
            name: name.join(" "),
            description,
        }
        .pipe(Ok)
    }
}

#[tokio::test]
#[cfg_attr(feature = "tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let step = to_stream_test! {
        ScriptExecution,
       r#"PhaseScriptExecution Format\ Swift\ Files /$ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Script-B78C717D92544DC366EB9EAB.sh (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT

"# 
    };
    assert_eq!("DemoTarget", &step.description.target);
    assert_eq!("DemoProject", &step.description.project);
    assert_eq!("Format Swift Files", step.name);
}
