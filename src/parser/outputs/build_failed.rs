use super::super::{Description, Error, OutputStream, ParsableFromStream};
use crate::parser::Step;
use async_trait::async_trait;
use process_stream::ProcessItem;
use std::{fmt::Display, path::PathBuf};
use tap::Pipe;
use tokio_stream::StreamExt;

#[derive(Debug)]
pub struct FailedCommand {
    pub kind: String,
    pub arch: String,
    pub file_path: PathBuf,
    pub file_name: String,
    pub target: String,
    pub project: String,
}
impl TryFrom<String> for FailedCommand {
    type Error = Error;

    fn try_from(line: String) -> Result<Self, Self::Error> {
        let mut chunks = line.split_whitespace();
        let kind = chunks
            .next()
            .ok_or_else(|| Error::EOF("FailedCommand".into(), "kind".into()))?
            .to_string();

        chunks.next();

        let arch = chunks
            .next()
            .ok_or_else(|| Error::EOF("FailedCommand".into(), "arch".into()))?
            .to_string();

        // TODO: make path relative to current directory
        let file_path = chunks
            .next()
            .ok_or_else(|| Error::EOF("FailedCommand".into(), "arch".into()))?
            .pipe(PathBuf::from);

        let file_name = file_path
            .file_name()
            .ok_or_else(|| Error::EOF("FailedCommand".into(), "file_name".into()))?
            .to_string_lossy()
            .to_string();

        let Description { project, target } = Description::from_chunks(chunks)?;

        Ok(Self {
            arch,
            kind,
            file_path,
            file_name,
            target,
            project,
        })
    }
}

#[derive(Debug)]
pub struct BuildFailed {
    pub commands: Vec<FailedCommand>,
}

#[async_trait]
impl ParsableFromStream for BuildFailed {
    async fn parse_from_stream(_: String, stream: &mut OutputStream) -> Result<Vec<Step>, Error> {
        let mut commands = vec![];
        stream.next().await;
        while let Some(ProcessItem::Output(line) | ProcessItem::Error(line)) = stream.next().await {
            if let Ok(command) = FailedCommand::try_from(line) {
                commands.push(command);
            }
        }
        Ok(vec![Step::BuildFailed(Self { commands })])
    }
}

impl Display for BuildFailed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[BuildFailed] Commands:")?;
        for FailedCommand {
            kind,
            target,
            file_name,
            ..
        } in self.commands.iter()
        {
            writeln!(f, "[BuildFailed] [{target}] {kind} {file_name}")?;
        }

        Ok(())
    }
}
