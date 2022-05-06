use std::fmt::Display;

use tap::Pipe;

use crate::parser::{Description, Error};

#[derive(Debug)]
/// Storyboard linked
pub struct LinkStoryboards {
    pub description: Description,
}

impl LinkStoryboards {
    pub fn new(line: String) -> Result<Self, Error> {
        Self {
            description: Description::from_line(line)?,
        }
        .pipe(Ok)
    }
}

impl Display for LinkStoryboards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} Linking     Storyboards", self.description)
    }
}
