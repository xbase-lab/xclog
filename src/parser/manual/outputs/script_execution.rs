use std::fmt::Display;

use crate::parser::{
    consume_till_empty_line, Description, Error, OutputStream, ParsableFromStream, Step,
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
    async fn parse_from_stream(
        line: String,
        stream: &mut OutputStream,
    ) -> Result<Vec<Step>, Error> {
        let mut steps = vec![];
        let mut chunks = line.split_whitespace();
        let mut name = vec![];

        // NOTE: relays on `/`
        while let Some(chunk) = chunks.next() {
            if chunk.starts_with("/") {
                break;
            }
            name.push(chunk.replace("\\", ""))
        }

        steps.push(Step::ScriptExecution(Self {
            name: name.join(" "),
            description: Description::from_line(line)?,
        }));

        steps.extend(consume_till_empty_line(stream).await);

        steps.pipe(Ok)
    }
}
impl Display for ScriptExecution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Executing `{}`", self.description, self.name)
    }
}

#[tokio::test]
#[cfg_attr(feature = "with_tracing", tracing_test::traced_test)]
async fn test() {
    use crate::parser::util::test::to_stream_test;

    let steps = to_stream_test! {
        ScriptExecution,
       r#"PhaseScriptExecution Format\ Swift\ Files /$ROOT/build/DemoTarget.build/Debug-iphoneos/DemoTarget.build/Script-B78C717D92544DC366EB9EAB.sh (in target 'DemoTarget' from project 'DemoProject')
    cd $ROOT

"# 
    };
    if let Step::ScriptExecution(step) = steps.first().unwrap() {
        assert_eq!("DemoTarget", &step.description.target);
        assert_eq!("DemoProject", &step.description.project);
        assert_eq!("Format Swift Files", step.name);
    } else {
        panic!("No script execution {steps:#?}")
    }
}
