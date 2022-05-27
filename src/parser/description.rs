use std::fmt::Display;

use super::Error;

#[derive(Debug)]
pub struct Description {
    pub project: String,
    pub target: String,
}

impl Description {
    /// Get target and project from line.
    pub fn from_line(line: String) -> Result<Self, Error> {
        let (target, project) = get_target_project(line.split_whitespace())?;

        Ok(Self { project, target })
    }

    pub(crate) fn from_chunks<'a>(chunks: std::str::SplitWhitespace<'a>) -> Result<Self, Error> {
        let (target, project) = get_target_project(chunks)?;

        Ok(Self { project, target })
    }
}

fn get_target_project<'a>(
    mut chunks: std::str::SplitWhitespace<'a>,
) -> Result<(String, String), Error> {
    let (mut project, mut target) = (None, None);

    while let Some(part) = chunks.next() {
        if part.contains("target") {
            target = chunks.next().map(|s| s.replace("'", ""));
        } else if part.contains("project") {
            project = chunks.next().map(|s| s.replace(")", "").replace("'", ""));
        }
    }

    let target = target.ok_or_else(|| Error::EOF("Description".into(), "target".into()))?;
    let project = project.ok_or_else(|| Error::EOF("Description".into(), "project".into()))?;

    Ok((target, project))
}

impl Display for Description {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.target,)
    }
}
